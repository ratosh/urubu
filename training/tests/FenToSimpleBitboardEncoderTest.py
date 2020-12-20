import unittest

from chess import parse_square, Piece

from training.network import FenToSimpleBitboardEncoder


class TestStringMethods(unittest.TestCase):

    def test_calculate_index_kings(self):
        encoder = FenToSimpleBitboardEncoder()
        square = parse_square("a8")
        index1 = encoder.calculate_index(square, Piece.from_symbol("k"))
        index2 = encoder.calculate_index(square, Piece.from_symbol("K"))
        self.assertEqual(index1, 56 + 64 * 5)
        self.assertEqual(index2, 56 + 64 * 5 + 384)

    def test_calculate_index_queens(self):
        encoder = FenToSimpleBitboardEncoder()
        square = parse_square("b8")
        index1 = encoder.calculate_index(square, Piece.from_symbol("q"))
        index2 = encoder.calculate_index(square, Piece.from_symbol("Q"))
        self.assertEqual(index1, square + 64 * 4)
        self.assertEqual(index2, square + 64 * 4 + 384)


if __name__ == '__main__':
    unittest.main()
