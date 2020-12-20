import argparse
import os

import torch
import yaml
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

    for board, result in train_loader:
        print(board.shape)
        print(result.shape)
    network = parser_factory.get(cfg).get_network()
    random_input = torch.randn(64, 768)
    print(network(random_input).shape)


if __name__ == "__main__":
    argparser = argparse.ArgumentParser(description='Pipeline for training.')
    argparser.add_argument('--cfg',
                           type=argparse.FileType('r'),
                           help='yaml configuration with training parameters')

    train(argparser.parse_args())
