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
from encoding import parser_factory
from network_pb2 import Network, Layer, Weight
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
                loop.set_postfix(lr=optimizer.param_groups[0]['lr'], loss=round(100*criterion_loss/(idx + 1), 5),
                                 acc=round(check_accuracy(validation_loader, network, cfg.device), 5))
            else:
                loop.set_postfix(lr=optimizer.param_groups[0]['lr'], loss=round(100*criterion_loss/(idx + 1), 5))

        scheduler.step(criterion_loss)

        network.to("cpu")
        torch.save({
            'network_state_dict': network.state_dict(),
            'optimizer_state_dict': optimizer.state_dict(),
            'scheduler_state_dict': scheduler.state_dict(),
        }, f"{cfg.output}\\epoch_{epoch + 1:06d}.chkp")
        network.to(cfg.device)

    proto_network = Network()
    for index, parameter in enumerate(network.parameters()):
        if parameter.dim() == 1:
            layer = proto_network.layers[-1]
            layer.bias[:] = [float(tensor.item()) for tensor in parameter.data]
        else:
            layer = Layer()
            for row in parameter.data:
                weight = Weight()
                weight.values[:] = [float(tensor.item()) for tensor in row.data]
                layer.weights.append(weight)
            proto_network.layers.append(layer)
    with open(f"{cfg.output}\\network.net", "wb") as file:
        file.write(proto_network.SerializeToString())

    if start_epoch < cfg.epochs:
        check_performance(validation_loader, network, cfg.device)


def check_performance(loader, model, device):
    print("Checking performance")
    error_map = {}
    score_map = {}
    result_map = {}
    model.eval()

    with torch.no_grad():
        for index, (input, result) in enumerate(loader):
            model_result = model(input.to(device))

            score_map[index] = float(model_result)
            result_map[index] = float(result)
            error_map[index] = abs(result_map[index] - score_map[index])

    sorted_keys = sorted(error_map, key=error_map.get, reverse=True)
    print(f"E:{(sum(error_map.values())/len(error_map))}")
    for key in sorted_keys[:10]:
        print(f"K:{key + 1} e:{error_map[key]} s:{score_map[key]} r:{result_map[key]}")
    model.train()


def check_accuracy(loader, model, device):
    model.eval()
    total_loss = 0.0

    with torch.no_grad():
        for index, (input, result) in enumerate(loader):
            model_result = model(input.to(device))
            total_loss += abs(float(result) - float(model_result))

    model.train()
    return total_loss/loader.__len__()


if __name__ == "__main__":
    argparser = argparse.ArgumentParser(description='Pipeline for training.')
    argparser.add_argument('--cfg',
                           type=argparse.FileType('r'),
                           help='yaml configuration with training parameters')

    train(argparser.parse_args())
