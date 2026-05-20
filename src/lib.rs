use pyo3::prelude::*;
use rayon::prelude::*;
// Import the stemmer implementation from the rust-stemmers library
extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};

// Create a Python class to expose the stemmer functionality
#[pyclass]
pub struct SnowballStemmer {
    stemmer: Stemmer,
}

#[pymethods]
impl SnowballStemmer {
    #[new]
    fn new(lang: &str) -> Self {
        let algorithm = match lang.to_lowercase().as_str() {
            "arabic" => Algorithm::Arabic,
            "danish" => Algorithm::Danish,
            "dutch" => Algorithm::Dutch,
            "english" => Algorithm::English,
            "finnish" => Algorithm::Finnish,
            "french" => Algorithm::French,
            "german" => Algorithm::German,
            "greek" => Algorithm::Greek,
            "hungarian" => Algorithm::Hungarian,
            "italian" => Algorithm::Italian,
            "norwegian" => Algorithm::Norwegian,
            "portuguese" => Algorithm::Portuguese,
            "romanian" => Algorithm::Romanian,
            "russian" => Algorithm::Russian,
            "spanish" => Algorithm::Spanish,
            "swedish" => Algorithm::Swedish,
            "tamil" => Algorithm::Tamil,
            "turkish" => Algorithm::Turkish,
            _ => panic!("Unsupported language: {}", lang),
        };
        let stemmer = Stemmer::create(algorithm);
        SnowballStemmer { stemmer }
    }

    #[inline(always)]
    fn stem_word(&self, input: &str) -> String {
        self.stemmer.stem(input).into_owned()
    }

    #[inline(always)]
    pub fn stem_words_parallel(&self, inputs: Vec<String>) -> Vec<String> {
        inputs
            .into_par_iter()
            .map(|word| self.stemmer.stem(&word).into_owned())
            .collect()
    }

    #[inline(always)]
    pub fn stem_words(&self, inputs: Vec<String>) -> Vec<String> {
        inputs
            .into_iter()
            .map(|word| self.stemmer.stem(&word).into_owned())
            .collect()
    }
}

/// This module is required for the Python interpreter to access the Rust functions.
#[pymodule]
fn py_rust_stemmers(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SnowballStemmer>()?;
    Ok(())
}
