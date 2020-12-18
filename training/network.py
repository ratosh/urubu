import chess
import torch
from torch import nn


class NetworkFactory:

    def __init__(self):
        self._network = {}

    def register_type(self, code, network):
        self._network[code] = network

    def get_network(self, cfg):
        code = cfg.model_type
        network = self._network.get(code)
        if not network:
            raise ValueError("Missing encoder for {}".format(code))
        return network(cfg)


network_factory = NetworkFactory()


class SimpleBitboardNetwork(nn.Module):

    def __init__(self, cfg):
        super(SimpleBitboardNetwork, self).__init__()
        self.cfg = cfg
        self.hidden = []
        for index, nodes in enumerate(cfg.model_dense_layout):
            if index == 0:
                self.input = nn.Linear(768, cfg.model_dense_layout[0])
            elif index == len(cfg.model_dense_layout) - 1:
                self.output = nn.Linear(cfg.model_dense_layout[index - 1], nodes)
            else:
                self.hidden.append(nn.Linear(cfg.model_dense_layout[index - 1], nodes))
        self.output = nn.Linear(32, 1)

    def forward(self, x):
        x = self.input(x)
        for hidden in self.hidden:
            x = torch.clamp(hidden(x), 0.0, 1.0)
        x = self.output(x)
        return x

    def encode(self, fen):
        # TODO: Encode NN input
        return str


network_factory.register_type(0, SimpleBitboardNetwork)
