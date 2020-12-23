import glob

from torch.utils.data import Dataset

from network import parser_factory


class CsvDataset(Dataset):

    def count_chunks_in_files(self, file):
        for line in open(file):
            self.items.append(line)

    def load_folder_files(self, directory):
        iterator = glob.iglob(directory + "*.csv")
        for file in iterator:
            self.count_chunks_in_files(file)

    def load_files(self, path):
        iterator = glob.iglob(path)
        for directory in iterator:
            self.load_folder_files(directory)

    def __init__(self, cfg, input_location):
        self.items = []
        self.encoded = {}
        self.encoder = parser_factory.get(cfg).get_encoder()
        self.load_files(input_location)
        self.len = len(self.items)

    def __len__(self):
        return self.len

    def encode(self, item):
        split = item.split(",")
        fen = split[0]
        result = split[1]
        return self.encoder.encode_fen(fen), self.encoder.encode_result(result)

    def __getitem__(self, index):
        if len(self.encoded) < index or index not in self.encoded:
            self.encoded[index] = self.encode(self.items[index])
        return self.encoded[index]
