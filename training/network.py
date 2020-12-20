import chess
import torch
from torch import nn


class ParserFactory:

    def __init__(self):
        self._network = {}

    def register(self, code, network):
        self._network[code] = network

    def get(self, cfg):
        code = cfg.model_type
        network = self._network.get(code)
        if not network:
            raise ValueError("Missing network encoder for {}".format(code))
        return network(cfg)


parser_factory = ParserFactory()


class SimpleBitboard:

    def __init__(self, cfg):
        self.network = SimpleBitboardNetwork(cfg)
        self.encoder = FenToSimpleBitboardEncoder()

    def get_network(self):
        return self.network

    def get_encoder(self):
        return self.encoder


parser_factory.register(0, SimpleBitboard)


class SimpleBitboardNetwork(nn.Module):

    def __init__(self, cfg):
        super(SimpleBitboardNetwork, self).__init__()
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


class FenToSimpleBitboardEncoder:

    def encode_fen(self, fen):
        board = chess.Board(fen=fen)
        result = torch.zeros(768)
        for (square, piece) in board.piece_map().items():
            result[self.calculate_index(square, piece)] = 1.0
        return result

    @staticmethod
    def encode_result(result):
        return torch.tensor([float(result)])

    @staticmethod
    def calculate_index(square: chess.Square, piece: chess.Piece):
        return square + 64 * (piece.piece_type - 1) + 64 * 6 * piece.color
