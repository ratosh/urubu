import argparse
import datetime
import os
import time

import yaml
from torch import nn, optim
from torch.utils.data import DataLoader

from training.dataset import CsvDataset
from training.network import parser_factory
from training_config import TrainingConfig


def train(args):
    yaml_file = yaml.safe_load(args.cfg)
    print(yaml.dump(yaml_file, default_flow_style=False))
    cfg = TrainingConfig(yaml_file)

    output_dir = cfg.output
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    train_dataset = CsvDataset(cfg, cfg.input)
    train_loader = DataLoader(dataset=train_dataset, batch_size=cfg.batch_size, shuffle=True)

    network = parser_factory.get(cfg).get_network()
    criterion = nn.BCEWithLogitsLoss()
    optimizer = optim.Adam(network.parameters(), lr=cfg.lr[0])
    lr_index = 0

    for step in range(cfg.steps):
        start_time = time.time()
        print("Starting step {} at {}.".format(step, datetime.datetime.now()))
        losses = []
        if lr_index < len(cfg.lr_bounds) and cfg.lr_bounds[lr_index] <= step:
            lr_index += 1
            optimizer.lr = cfg.lr[lr_index]
        print("Learning rate {}".format(cfg.lr[lr_index]))

        for batch_idx, (data, targets) in enumerate(train_loader):
            print("Batch {}".format(batch_idx))
            scores = network(data)
            loss = criterion(scores, targets)

            losses.append(loss.item())

            # backward
            optimizer.zero_grad()
            loss.backward()

            optimizer.step()
            print("Batch loss {}".format(loss.item()))

        elapsed_time = time.time() - start_time
        print("Step completed")
        print("Loss: {}".format(sum(losses)/len(losses)))
        print("Time: {}.".format(elapsed_time))


if __name__ == "__main__":
    argparser = argparse.ArgumentParser(description='Pipeline for training.')
    argparser.add_argument('--cfg',
                           type=argparse.FileType('r'),
                           help='yaml configuration with training parameters')

    train(argparser.parse_args())
