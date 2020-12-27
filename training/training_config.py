import os

import torch


class TrainingConfig:

    # name: 'name'
    # device: cpu/cuda
    # training:
    #   batch_size: 1024
    #   input: '.\input_sample\'
    #   output: '.\training_output\'
    #   validation_sample: '.\validation_sample\'
    #   steps: 100000
    #   learning_rate: 0.01
    # model:
    #   type: 0
    #   dense:
    #       - 32
    #       - 32
    #       - 1
    def __init__(self, yaml_file):
        self.device = torch.device(yaml_file.get('device', 'cpu'))
        self.batch_size = yaml_file['training'].get('batch_size', 1024)
        self.input = yaml_file['training']['input']
        self.output = os.path.join(yaml_file['training']['output'], yaml_file['name'])
        self.validation = yaml_file['training']['validation']
        self.steps = yaml_file['training'].get('steps', 100000)
        self.lr = yaml_file['training'].get('learning_rate', 0.01)
        self.model_type = yaml_file['model'].get('type', 0)
        self.model_dense_layout = yaml_file['model'].get('dense', [32, 32, 1])
