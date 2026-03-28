from typing import List, Sequence

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

    def stem_words(self, inputs: Sequence[str]) -> List[str]:
        """
        Stem a list of words sequentially.
        
        Args:
            inputs: A list of words to stem.
        
        Returns:
            A list of stemmed words.
        """
        ...

    def stem_words_parallel(self, inputs: Sequence[str]) -> List[str]:
        """
        Stem a list of words in parallel using Rayon.
        
        Args:
            inputs: A list of words to stem.
        
        Returns:
            A list of stemmed words.
        """
        ...
