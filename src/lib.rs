use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use rayon::prelude::*;
use rust_stemmers::{Algorithm, Stemmer};
use std::borrow::Cow;

/// Lookup table mapping language names (lowercase)
const SUPPORTED_LANGUAGES: &[(&str, Algorithm)] = &[
    ("arabic", Algorithm::Arabic),
    ("danish", Algorithm::Danish),
    ("dutch", Algorithm::Dutch),
    ("english", Algorithm::English),
    ("finnish", Algorithm::Finnish),
    ("french", Algorithm::French),
    ("german", Algorithm::German),
    ("greek", Algorithm::Greek),
    ("hungarian", Algorithm::Hungarian),
    ("italian", Algorithm::Italian),
    ("norwegian", Algorithm::Norwegian),
    ("portuguese", Algorithm::Portuguese),
    ("romanian", Algorithm::Romanian),
    ("russian", Algorithm::Russian),
    ("spanish", Algorithm::Spanish),
    ("swedish", Algorithm::Swedish),
    ("tamil", Algorithm::Tamil),
    ("turkish", Algorithm::Turkish),
];

#[pyclass]
pub struct SnowballStemmer {
    stemmer: Stemmer,
}

impl SnowballStemmer {
    fn stem_normalized(&self, input: &str) -> String {
        let normalized = if input.chars().any(char::is_uppercase) {
            Cow::Owned(input.to_lowercase())
        } else {
            Cow::Borrowed(input)
        };
        self.stemmer.stem(&normalized).into_owned()
    }
}

#[pymethods]
impl SnowballStemmer {
    /// Creates a new instance of the stemmer for the specified language.
    #[new]
    pub fn new(lang: &str) -> PyResult<Self> {
        let algorithm = SUPPORTED_LANGUAGES
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(lang))
            .map(|&(_, algo)| algo)
            .ok_or_else(|| {
                let supported = SUPPORTED_LANGUAGES
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>()
                    .join(", ");
                PyValueError::new_err(format!(
                    "Unsupported language: '{}'. Supported languages are: {}",
                    lang, supported
                ))
            })?;

        Ok(Self {
            stemmer: Stemmer::create(algorithm),
        })
    }

    /// Stems a single word.
    pub fn stem_word(&self, input: &str) -> String {
        self.stem_normalized(input)
    }

    /// Stems an iterable of words sequentially.
    pub fn stem_words(&self, inputs: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
        let iter = inputs.try_iter()?;
        let mut results = Vec::with_capacity(inputs.len().unwrap_or(0));
        for item in iter {
            let word: String = item?.extract()?;
            results.push(self.stem_normalized(&word));
        }
        Ok(results)
    }

    /// Stems an iterable of words in parallel. The GIL is released during processing.
    pub fn stem_words_parallel(
        &self,
        py: Python<'_>,
        inputs: &Bound<'_, PyAny>,
    ) -> PyResult<Vec<String>> {
        let words: Vec<String> = inputs
            .try_iter()?
            .map(|item| item?.extract::<String>())
            .collect::<PyResult<_>>()?;

        Ok(py.detach(|| {
            words
                .into_par_iter()
                .with_min_len(1024)
                .map(|word| self.stem_normalized(&word))
                .collect()
        }))
    }
}

#[pymodule]
fn py_rust_stemmers(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SnowballStemmer>()?;
    Ok(())
}