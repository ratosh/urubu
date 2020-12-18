import argparse

import torch
import yaml
import glob
import os

from training.network import NetworkFactory, network_factory
from training_config import TrainingConfig


def count_chunks_in_files(file):
    chunks = 0
    for line in open(file):
        chunks += 1
    return chunks


def check_folder_files(directory):
    files = {}
    iterator = glob.iglob(directory + "*.csv")
    for file in iterator:
        files[file] = count_chunks_in_files(file)
    return files


def check_all_folders(path):
    chunks = {}
    iterator = glob.iglob(path)
    for directory in iterator:
        chunks = {**chunks, **check_folder_files(directory)}
    return chunks


def train(args):
    yaml_file = yaml.safe_load(args.cfg)
    print(yaml.dump(yaml_file, default_flow_style=False))
    cfg = TrainingConfig(yaml_file)

    output_dir = cfg.output
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    chunks = check_all_folders(cfg.input)
    total_chunks = sum(chunks.values())
    print("Found {} chunks".format(total_chunks))
    network = network_factory.get_network(cfg)
    random_input = torch.randn(64, 768)
    print(network(random_input).shape)


if __name__ == "__main__":
    argparser = argparse.ArgumentParser(description='Pipeline for training.')
    argparser.add_argument('--cfg',
                           type=argparse.FileType('r'),
                           help='yaml configuration with training parameters')

    train(argparser.parse_args())
