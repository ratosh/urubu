import unittest

from chess import parse_square, Piece

from training.encoding import FenToSimpleBitboardEncoder


class TestStringMethods(unittest.TestCase):

    def test_calculate_index_kings(self):
        encoder = FenToSimpleBitboardEncoder()
        square1 = parse_square("a8")
        square2 = parse_square("a1")
        index1 = encoder.calculate_index(square1, Piece.from_symbol("k"), 0)
        index2 = encoder.calculate_index(square1, Piece.from_symbol("K"), 0)
        index3 = encoder.calculate_index(square2, Piece.from_symbol("k"), 1)
        index4 = encoder.calculate_index(square2, Piece.from_symbol("K"), 1)
        self.assertEqual(square1 + 64 * 10, index1)
        self.assertEqual(square1 + 64 * 11, index2)
        self.assertEqual(index2, index3)
        self.assertEqual(index1, index4)

    def test_calculate_index_queens(self):
        encoder = FenToSimpleBitboardEncoder()
        square1 = parse_square("b8")
        square2 = parse_square("b1")
        index1 = encoder.calculate_index(square1, Piece.from_symbol("q"), 0)
        index2 = encoder.calculate_index(square1, Piece.from_symbol("Q"), 0)
        index3 = encoder.calculate_index(square2, Piece.from_symbol("q"), 1)
        index4 = encoder.calculate_index(square2, Piece.from_symbol("Q"), 1)
        self.assertEqual(square1 + 64 * 8, index1)
        self.assertEqual(square1 + 64 * 9, index2)
        self.assertEqual(index2, index3)
        self.assertEqual(index1, index4)


if __name__ == '__main__':
    unittest.main()
