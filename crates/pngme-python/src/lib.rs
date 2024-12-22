#[pyo3::pymodule(gil_used = false)]
#[pyo3(name = "pngme")]
mod pngme_python {
    use pyo3::exceptions::PyFileNotFoundError;
    use pyo3::{prelude::*, PyResult};
    use std::path::PathBuf;

    use pngme_lib;

    #[pyfunction]
    pub fn encode(path: PathBuf, chunk_type: String, message: String) -> PyResult<()> {
        let result = pngme_lib::encode(path, chunk_type, message);
        match result {
            Ok(_) => Ok(()),
            // TODO: Map actual lib error types to Python exceptions
            Err(e) => Err(PyFileNotFoundError::new_err(e.to_string())),
        }
    }

    #[pyfunction]
    pub fn decode(path: PathBuf, chunk_type: String) -> PyResult<String> {
        let result = pngme_lib::decode(path, chunk_type);
        match result {
            Ok(msg) => Ok(msg),
            // TODO: Map actual lib error types to Python exceptions
            Err(e) => Err(PyFileNotFoundError::new_err(e.to_string())),
        }
    }

    #[pyfunction]
    pub fn remove(path: PathBuf, chunk_type: String) -> PyResult<()> {
        let result = pngme_lib::remove(path, chunk_type);
        match result {
            Ok(_) => Ok(()),
            // TODO: Map actual lib error types to Python exceptions
            Err(e) => Err(PyFileNotFoundError::new_err(e.to_string())),
        }
    }
}
