import unittest
from py_rust_stemmers import SnowballStemmer

class TestRustStemmer(unittest.TestCase):
      
    def test_english_stemming(self):
        s = SnowballStemmer('english')
        words = ["fruitlessly", "happiness", "computations"]
        expected = ["fruitless", "happi", "comput"]
        result = [s.stem_word(w) for w in words]
        self.assertEqual(result, expected)

    def test_spanish_stemming(self):
        s = SnowballStemmer('spanish')
        words = ["frutalmente", "felicidad", "computaciones"]
        expected = ["frutal", "felic", "comput"]
        result = [s.stem_word(w) for w in words]
        self.assertEqual(result, expected)

    def test_empty_input(self):
        s = SnowballStemmer('english')
        expected = ['']
        result = [s.stem_word("")]
        self.assertEqual(result, expected)

    def test_invalid_language(self):
        words = ["fruitlessly", "happiness", "computations"]
        with self.assertRaises(ValueError):
            s = SnowballStemmer('invalid_lang')

if __name__ == '__main__':
    unittest.main()
