import argparse
import glob
import os
import re
import torch
import yaml

from torch import nn, optim
from torch.utils.data import DataLoader
from tqdm import tqdm

from dataset import CsvDataset
from network import parser_factory
from training_config import TrainingConfig


def train(args):
    yaml_file = yaml.safe_load(args.cfg)
    print(yaml.dump(yaml_file, default_flow_style=False))
    cfg = TrainingConfig(yaml_file)

    output_dir = cfg.output
    start_step = 0
    network = parser_factory.get(cfg).get_network()

    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    else:
        to_load = ""
        iterator = glob.iglob(output_dir + "\\*.net")
        for file in iterator:
            digit = re.findall(r'\d+', file)[0]
            number = int(digit)
            if start_step < number:
                start_step = number
                to_load = file
        if to_load:
            print(f"Loading previous run: {to_load}")
            network.load_state_dict(torch.load(to_load))

    torch.set_num_threads(cfg.threads)
    train_dataset = CsvDataset(cfg, cfg.input)
    train_loader = DataLoader(dataset=train_dataset, batch_size=cfg.batch_size, shuffle=True)

    criterion = nn.MSELoss(reduction="sum")
    optimizer = optim.Adam(network.parameters(), lr=cfg.lr[0])
    lr_index = 0
    network.to(cfg.device)

    for step in range(start_step, cfg.steps):
        losses = []
        if lr_index < len(cfg.lr_bounds) and cfg.lr_bounds[lr_index] <= step:
            lr_index += 1
            optimizer.lr = cfg.lr[lr_index]

        loop = tqdm(enumerate(train_loader), total=len(train_loader))
        for idx, (data, targets) in loop:
            scores = network(data.to(cfg.device))
            loss = criterion(scores, targets.to(cfg.device))

            losses.append(loss.item())

            # backward
            optimizer.zero_grad()
            loss.backward()

            optimizer.step()
            loop.set_description(f"STEP [{step + 1}/{cfg.steps}] LR {cfg.lr[lr_index]}")
            loop.set_postfix(loss=sum(losses)/len(losses))

        torch.save(network.state_dict(), f"{cfg.output}/step_{step + 1:06d}.net")

    validation_dataset = CsvDataset(cfg, cfg.validation)
    validation_loader = DataLoader(dataset=validation_dataset)
    # check_accuracy(train_loader, network, cfg.device)
    network.eval()
    check_accuracy(validation_loader, network, cfg.device)

    # TODO: Save network


def check_accuracy(loader, model, device):
    error = 0
    num_samples = 0
    model.eval()

    with torch.no_grad():
        for x, y in loader:
            x = x.to(device=device)
            y = y.to(device=device)

            score = model(x)
            error += 1 - abs(y - score)
            num_samples += 1

        print(f"{num_samples} samples with accuracy {float(error)/float(num_samples)*100:.2f}%")

    model.train()


if __name__ == "__main__":
    argparser = argparse.ArgumentParser(description='Pipeline for training.')
    argparser.add_argument('--cfg',
                           type=argparse.FileType('r'),
                           help='yaml configuration with training parameters')

    train(argparser.parse_args())
