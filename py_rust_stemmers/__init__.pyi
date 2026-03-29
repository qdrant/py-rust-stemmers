from typing import Iterable, List, TypeVar

_S = TypeVar("_S", bound=str)

class SnowballStemmer:
    """
    A fast and parallel Snowball stemmer implemented in Rust.
    """
    def __init__(self, lang: str) -> None:
        """
        Create a new SnowballStemmer for the specified language.
        
        Args:
            lang: The language to use (e.g., 'english', 'spanish').
        
        Raises:
            ValueError: If the language is not supported.
        """
        ...

    def stem_word(self, input: str) -> str:
        """
        Stem a single word.
        
        Args:
            input: The word to stem.
        
        Returns:
            The stemmed word.
        """
        ...

    def stem_words(self, inputs: Iterable[_S]) -> List[str]:
        """
        Stem an iterable of words sequentially.
        
        Args:
            inputs: An iterable of words to stem.
        
        Returns:
            A list of stemmed words.
        """
        ...

    def stem_words_parallel(self, inputs: Iterable[_S]) -> List[str]:
        """
        Stem an iterable of words in parallel using Rayon.
        
        Args:
            inputs: An iterable of words to stem.
        
        Returns:
            A list of stemmed words.
        """
        ...
