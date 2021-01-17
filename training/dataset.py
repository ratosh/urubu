import glob

from torch.utils.data import Dataset

from encoding import parser_factory


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
        self.encoder = parser_factory.get(cfg).get_encoder()
        self.load_files(input_location)

    def __len__(self):
        return len(self.items)

    def encode(self, item):
        split = item.split(",")
        fen = split[0]
        result = split[1]
        encoded_fen = self.encoder.encode_fen(fen)
        return encoded_fen[0], self.encoder.encode_result(result, encoded_fen[1])

    def __getitem__(self, index):
        return self.encode(self.items[index])
