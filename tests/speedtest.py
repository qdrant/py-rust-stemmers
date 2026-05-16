import time
from py_rust_stemmers import SnowballStemmer
from snowballstemmer import stemmer

print(f'Init at {time.perf_counter()}')
# Create an instance of the stemmer for the English language
s = SnowballStemmer('english')
text = """This stem form is often a word itself, but this is not always the case as this is not a requirement for text search systems, which are the intended field of use. We also aim to conflate words with the same meaning, rather than all words with a common linguistic root (so awe and awful don't have the same stem), and over-stemming is more problematic than under-stemming so we tend not to stem in cases that are hard to resolve. If you want to always reduce words to a root form and/or get a root form which is itself a word then Snowball's stemming algorithms likely aren't the right answer."""
words = text.split()

loops = 500000
a = time.perf_counter()
for _ in range(loops):
    for word in words:
        stemmed = s.stem_word(word)
print("Time taken py_rust_stemmers:", time.perf_counter() - a)

d = time.perf_counter()

for _ in range(int(loops / 100)):
    stemmed_words = s.stem_words_parallel("".join([text for i in range(100)]).split())

print("Parallel:", time.perf_counter() - d)

d = time.perf_counter()

for _ in range(loops):
    stemmed_words = s.stem_words(text.split())

print("Non parallel:", time.perf_counter() - d)

s = stemmer('english')
b = time.perf_counter()
# only time a fraction of these, 500k takes 10 minutes
for _ in range(loops // 100):
    for word in words:
        stemmed = s.stemWord(word)
print("Time taken snowballstemmer with PyStemmer installed:", (time.perf_counter() - b) * 100)

