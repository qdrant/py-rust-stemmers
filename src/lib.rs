use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use rayon::prelude::*;
use rust_stemmers::{Algorithm, Stemmer};

#[pyclass]
pub struct SnowballStemmer {
    stemmer: Stemmer,
}

#[pymethods]
impl SnowballStemmer {
    /// Creates a new instance of the stemmer for the specified language.
    /// 
    /// Example:
    ///     >>> stemmer = SnowballStemmer("portuguese")
    #[new]
    fn new(lang: &str) -> PyResult<Self> {
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
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported language: {}",
                    lang
                )))
            }
        };
        let stemmer = Stemmer::create(algorithm);
        Ok(SnowballStemmer { stemmer })
    }

    /// Stems a single word.
    pub fn stem_word(&self, input: &str) -> String {
        self.stemmer.stem(input).into_owned()
    }

    /// Stems a list of words using multiple threads.
    /// Ideal for large batches. The GIL is released.
    pub fn stem_words_parallel(&self, py: Python<'_>, inputs: Vec<String>) -> Vec<String> {
        py.detach(|| {
            inputs
                .into_par_iter()
                .map(|word| self.stemmer.stem(&word).into_owned())
                .collect()
        })
    }

    /// Stems a list of words sequentially.
    /// Ideal for smaller lists.
    pub fn stem_words(&self, inputs: &Bound<'_, PyList>) -> PyResult<Vec<String>> {
        let mut results = Vec::with_capacity(inputs.len());
        
        for item in inputs {
            let word: String = item.extract()?;
            results.push(self.stemmer.stem(&word).into_owned());
        }
        
        Ok(results)
    }
}

#[pymodule]
fn py_rust_stemmers(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SnowballStemmer>()?;
    Ok(())
}
