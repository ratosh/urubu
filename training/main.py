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
    start_epoch = 0
    network = parser_factory.get(cfg).get_network()
    network.to(cfg.device)

    criterion_function = nn.MSELoss()
    optimizer = optim.Adagrad(network.parameters(), lr=cfg.lr)

    scheduler = optim.lr_scheduler.ReduceLROnPlateau(optimizer, factor=0.5, patience=5, verbose=False)

    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    else:
        to_load = ""
        iterator = glob.iglob(f"{output_dir}\\*.chkp")
        for file in iterator:
            digit = re.findall(r'\d+', file)[-1]
            number = int(digit)
            if start_epoch < number:
                start_epoch = number
                to_load = file
        if to_load:
            print(f"Loading previous run: {to_load}")
            file_loaded = torch.load(to_load)
            network.to("cpu")
            network.load_state_dict(file_loaded["network_state_dict"])
            network.to(cfg.device)
            optimizer.load_state_dict(file_loaded["optimizer_state_dict"])
            scheduler.load_state_dict(file_loaded["scheduler_state_dict"])
            network.train()

    train_dataset = CsvDataset(cfg, cfg.input)
    train_loader = DataLoader(dataset=train_dataset, batch_size=cfg.batch_size, shuffle=True, num_workers=cfg.threads)

    validation_dataset = CsvDataset(cfg, cfg.validation)
    validation_loader = DataLoader(dataset=validation_dataset, num_workers=cfg.threads)

    if start_epoch != 0:
        check_performance(validation_loader, network, cfg.device)
    for epoch in range(start_epoch, cfg.epochs):
        criterion_loss = 0

        loop = tqdm(enumerate(train_loader), total=len(train_loader))
        loop.set_description(f"EPOCH [{epoch + 1}/{cfg.epochs}]")
        for idx, (data, targets) in loop:
            optimizer.zero_grad()
            targets = targets.to(cfg.device)
            loss = criterion_function(network(data.to(cfg.device)), targets)

            criterion_loss += loss.item()

            # backward
            loss.backward()
            optimizer.step()

            if (epoch + 1) % 10 == 0 and idx == loop.total - 1:
                loop.set_postfix(lr=optimizer.param_groups[0]['lr'], loss=round(100*criterion_loss/(idx + 1), 3),
                                 acc=round(check_accuracy(validation_loader, network, cfg.device), 3))
            else:
                loop.set_postfix(lr=optimizer.param_groups[0]['lr'], loss=round(100*criterion_loss/(idx + 1), 3))

        scheduler.step(criterion_loss)

        network.to("cpu")
        torch.save({
            'network_state_dict': network.state_dict(),
            'optimizer_state_dict': optimizer.state_dict(),
            'scheduler_state_dict': scheduler.state_dict(),
        }, f"{cfg.output}\\epoch_{epoch + 1:06d}.chkp")
        network.to(cfg.device)
    check_performance(validation_loader, network, cfg.device)

    # TODO: Save network binary (not torch)


def check_performance(loader, model, device):
    print("Checking performance")
    error_map = {}
    score_map = {}
    result_map = {}
    model.eval()

    with torch.no_grad():
        for index, (x, y) in enumerate(loader):
            x = x.to(device=device)
            y = y.to(device=device)

            score = model(x)
            error = abs(float(y) - float(score))
            error_map[index] = error
            score_map[index] = float(score)
            result_map[index] = float(y)

    sorted_keys = sorted(error_map, key=error_map.get, reverse=True)
    print(f"E:{100*sum(error_map.values())/len(error_map)}")
    for key in sorted_keys[:10]:
        print(f"I:{key + 1} e:{error_map[key]} s:{score_map[key]} r:{result_map[key]}")
    model.train()


def check_accuracy(loader, model, device):
    error_map = {}
    model.eval()

    with torch.no_grad():
        for index, (x, y) in enumerate(loader):
            x = x.to(device=device)
            y = y.to(device=device)

            score = model(x)
            error = abs(float(y) - float(score))
            error_map[index] = error

    model.train()
    return 100*sum(error_map.values())/len(error_map)


if __name__ == "__main__":
    argparser = argparse.ArgumentParser(description='Pipeline for training.')
    argparser.add_argument('--cfg',
                           type=argparse.FileType('r'),
                           help='yaml configuration with training parameters')

    train(argparser.parse_args())
