import unittest
from py_rust_stemmers import SnowballStemmer

class TestRustStemmer(unittest.TestCase):

    def test_english_stemming(self):
        words = ["fruitlessly", "happiness", "computations"]
        expected = ["fruitless", "happi", "comput"]
        stemmer = SnowballStemmer("english")
        result = stemmer.stem_words(words)
        self.assertEqual(result, expected)

    def test_spanish_stemming(self):
        words = ["frutalmente", "felicidad", "computaciones"]
        expected = ["frutal", "felic", "comput"]
        stemmer = SnowballStemmer("spanish")
        result = stemmer.stem_words(words)
        self.assertEqual(result, expected)

    def test_empty_input(self):
        words = []
        expected = []
        stemmer = SnowballStemmer("english")
        result = stemmer.stem_words(words)
        self.assertEqual(result, expected)

    def test_invalid_language(self):
        words = ["fruitlessly", "happiness", "computations"]
        # SnowballStemmer("invalid_lang") should raise an exception (Panic results in RuntimeError in PyO3)
        with self.assertRaises((ValueError, RuntimeError, Exception)):
            SnowballStemmer("invalid_lang")

if __name__ == '__main__':
    unittest.main()
