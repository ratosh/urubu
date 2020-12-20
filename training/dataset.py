import glob

from torch.utils.data import Dataset

from training.network import parser_factory


class CsvDataset(Dataset):

    @staticmethod
    def count_chunks_in_files(file):
        chunks = 0
        for line in open(file):
            chunks += 1
        return chunks

    def check_folder_files(self, directory):
        files = {}
        iterator = glob.iglob(directory + "*.csv")
        for file in iterator:
            files[file] = self.count_chunks_in_files(file)
        return files

    def check_all_folders(self, path):
        chunks = {}
        iterator = glob.iglob(path)
        for directory in iterator:
            chunks = {**chunks, **self.check_folder_files(directory)}
        return chunks

    def __init__(self, cfg, input_location):
        self.file_line_count = self.check_all_folders(input_location)
        self.len = sum(self.file_line_count.values())
        self.encoder = parser_factory.get(cfg).get_encoder()
        print("Found {} chunks".format(self.len))

    def __len__(self):
        return self.len

    def __getitem__(self, index):
        fen = {}
        result = 0.0
        for (file_name, count) in self.file_line_count.items():
            if index < count:
                with open(file_name) as file:
                    for skip_lines in range(index):
                        file.readline()
                    line = file.readline()
                    split = line.split(",")
                    fen = split[0]
                    result = split[1]
            index -= count

        return self.encoder.encode_fen(fen), self.encoder.encode_result(result)
