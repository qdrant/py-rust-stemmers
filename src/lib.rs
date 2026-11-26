use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rayon::prelude::*;
use rust_stemmers::{Algorithm, Stemmer};
use std::sync::atomic::{AtomicU32, Ordering};

/// PID of the process that started rayon's global thread pool. A fork()ed child inherits
/// the pool's state but not its threads, so handing it work there would block forever.
static RAYON_PID: AtomicU32 = AtomicU32::new(0);

fn rayon_usable() -> bool {
    let pid = std::process::id();
    match RAYON_PID.compare_exchange(0, pid, Ordering::Relaxed, Ordering::Relaxed) {
        Ok(_) => true,
        Err(owner) => owner == pid,
    }
}

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

#[pymethods]
impl SnowballStemmer {
    #[new]
    fn new(lang: &str) -> PyResult<Self> {
        let lower = lang.to_lowercase();
        let algorithm = SUPPORTED_LANGUAGES
            .iter()
            .find(|(name, _)| *name == lower.as_str())
            .map(|(_, algo)| *algo)
            .ok_or_else(|| {
                let supported = SUPPORTED_LANGUAGES
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>()
                    .join(", ");
                PyValueError::new_err(format!(
                    "Unsupported language: '{lang}'. Supported languages are: {supported}."
                ))
            })?;
        Ok(SnowballStemmer {
            stemmer: Stemmer::create(algorithm),
        })
    }

    #[inline(always)]
    fn stem_word(&self, input: &str) -> String {
        self.stemmer.stem(input).into_owned()
    }

    #[inline(always)]
    pub fn stem_words_parallel(&self, inputs: Vec<String>) -> Vec<String> {
        if !rayon_usable() {
            return self.stem_words(inputs);
        }
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
