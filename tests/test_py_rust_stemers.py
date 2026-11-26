import os
import signal
import time
import warnings

import pytest

import py_rust_stemmers


@pytest.fixture
def english():
    return py_rust_stemmers.SnowballStemmer("english")


def test_english_stem_word(english):
    assert english.stem_word("fruitlessly") == "fruitless"


def test_english_stem_words(english):
    words = ["fruitlessly", "happiness", "computations"]
    assert english.stem_words(words) == ["fruitless", "happi", "comput"]


def test_english_stem_words_parallel(english):
    words = ["fruitlessly", "happiness", "computations"]
    assert english.stem_words_parallel(words) == ["fruitless", "happi", "comput"]


def test_parallel_matches_sequential(english):
    words = ["fruitlessly", "happiness", "computations", "running", "tested"]
    assert english.stem_words_parallel(words) == english.stem_words(words)


@pytest.mark.skipif(not hasattr(os, "fork"), reason="needs os.fork")
def test_stem_words_parallel_in_forked_child(english):
    words = ["fruitlessly", "happiness", "computations"] * 1000
    expected = english.stem_words_parallel(words)  # starts rayon's thread pool in this process
    with warnings.catch_warnings():
        warnings.simplefilter("ignore", DeprecationWarning)  # fork() in a multi-threaded process
        pid = os.fork()
    if pid == 0:  # child: never return into pytest
        try:
            os._exit(0 if english.stem_words_parallel(words) == expected else 1)
        finally:
            os._exit(1)
    deadline = time.monotonic() + 10
    while time.monotonic() < deadline:
        done, status = os.waitpid(pid, os.WNOHANG)
        if done:
            assert os.waitstatus_to_exitcode(status) == 0
            return
        time.sleep(0.05)
    os.kill(pid, signal.SIGKILL)
    os.waitpid(pid, 0)
    pytest.fail("stem_words_parallel hung in a forked child")


def test_spanish_stem_words():
    stemmer = py_rust_stemmers.SnowballStemmer("spanish")
    words = ["frutalmente", "felicidad", "computaciones"]
    assert stemmer.stem_words(words) == ["frutal", "felic", "comput"]


def test_empty_input(english):
    assert english.stem_words([]) == []


def test_empty_input_parallel(english):
    assert english.stem_words_parallel([]) == []


def test_invalid_language_raises_value_error():
    with pytest.raises(ValueError, match="Unsupported language"):
        py_rust_stemmers.SnowballStemmer("invalid_lang")


def test_invalid_language_error_message_contains_supported_languages():
    with pytest.raises(ValueError, match="english"):
        py_rust_stemmers.SnowballStemmer("invalid_lang")


def test_language_is_case_insensitive():
    stemmer = py_rust_stemmers.SnowballStemmer("English")
    assert stemmer.stem_word("fruitlessly") == "fruitless"


def test_non_string_word_raises_type_error(english):
    with pytest.raises(TypeError):
        english.stem_words(["fruitlessly", 123, "computations"])


def test_non_string_language_raises_type_error():
    with pytest.raises(TypeError):
        py_rust_stemmers.SnowballStemmer(123)
