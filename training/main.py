import argparse
import glob
import operator
import os
import re
from collections import OrderedDict

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
            digit = re.findall(r'\d+', file)[-1]
            number = int(digit)
            if start_step < number:
                start_step = number
                to_load = file
        if to_load:
            print(f"Loading previous run: {to_load}")
            network.load_state_dict(torch.load(to_load))
    network.to(cfg.device)

    train_dataset = CsvDataset(cfg, cfg.input)
    train_loader = DataLoader(dataset=train_dataset, batch_size=cfg.batch_size, shuffle=True)

    validation_dataset = CsvDataset(cfg, cfg.validation)
    validation_loader = DataLoader(dataset=validation_dataset)

    criterion_function = nn.MSELoss()
    optimizer = optim.Adam(network.parameters(), lr=cfg.lr)

    scheduler = optim.lr_scheduler.ReduceLROnPlateau(optimizer, factor=0.5, patience=5, verbose=False)

    for step in range(start_step, cfg.steps):
        criterion_loss = 0

        loop = tqdm(enumerate(train_loader), total=len(train_loader))
        loop.set_description(f"STEP [{step + 1}/{cfg.steps}]")
        for idx, (data, targets) in loop:
            optimizer.zero_grad()
            targets = targets.to(cfg.device)
            loss = criterion_function(network(data.to(cfg.device)), targets)

            criterion_loss += loss.item()

            # backward
            loss.backward()
            optimizer.step()

            loop.set_postfix(lr=optimizer.param_groups[0]['lr'], loss=round(100*criterion_loss/(idx + 1), 2))

        scheduler.step(criterion_loss)
        torch.save(network.state_dict(), f"{cfg.output}/step_{step + 1:06d}.net")
    check_performance(validation_loader, network, cfg.device)

    # TODO: Save network


def check_performance(loader, model, device):
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
    print(f"E:{sum(error_map.values())/len(error_map)}")
    for key in sorted_keys[:10]:
        print(f"I:{key + 1} e:{error_map[key]} s:{score_map[key]} r:{result_map[key]}")
    model.train()


if __name__ == "__main__":
    argparser = argparse.ArgumentParser(description='Pipeline for training.')
    argparser.add_argument('--cfg',
                           type=argparse.FileType('r'),
                           help='yaml configuration with training parameters')

    train(argparser.parse_args())
