use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "pngme")]
mod pngme_python {
    use pyo3::exceptions::{PyFileNotFoundError, PyIOError, PyValueError};
    use pyo3::{prelude::*, PyResult};
    use std::path::PathBuf;

    use pngme_lib::{decode as png_decode, encode as png_encode, remove as png_remove, Error};

    #[pyfunction]
    pub fn encode(path: PathBuf, chunk_type: String, message: String) -> PyResult<()> {
        let result = png_encode(path, chunk_type, message);
        match result {
            Ok(_) => Ok(()),
            Err(e) => match e {
                Error::FileNotFound { source: _, path: _ } => {
                    Err(PyFileNotFoundError::new_err(e.to_string()))
                }
                Error::Read { source: s } => Err(PyIOError::new_err(s.to_string())),
                Error::PNGParse => Err(PyValueError::new_err(e.to_string())),
                Error::InvalidChunkType {
                    chunk_type: _,
                    source: s,
                } => Err(PyValueError::new_err(s.to_string())),
                Error::PNGWrite { source: _ } => Err(PyValueError::new_err(e.to_string())),
                Error::ChunkNotFound { chunk_type: _ } => Err(PyValueError::new_err(e.to_string())),
                Error::StrConversion => Err(PyValueError::new_err(e.to_string())),
            },
        }
    }

    #[pyfunction]
    pub fn decode(path: PathBuf, chunk_type: String) -> PyResult<String> {
        let result = png_decode(path, chunk_type);
        match result {
            Ok(msg) => Ok(msg),
            Err(e) => match e {
                Error::FileNotFound { source: _, path: _ } => {
                    Err(PyFileNotFoundError::new_err(e.to_string()))
                }
                Error::Read { source: _ } => Err(PyIOError::new_err(e.to_string())),
                Error::PNGParse => Err(PyValueError::new_err(e.to_string())),
                Error::InvalidChunkType {
                    chunk_type: _,
                    source: s,
                } => Err(PyValueError::new_err(s.to_string())),
                Error::PNGWrite { source: _ } => Err(PyValueError::new_err(e.to_string())),
                Error::ChunkNotFound { chunk_type: _ } => Err(PyValueError::new_err(e.to_string())),
                Error::StrConversion => Err(PyValueError::new_err(e.to_string())),
            },
        }
    }

    #[pyfunction]
    pub fn remove(path: PathBuf, chunk_type: String) -> PyResult<()> {
        let result = png_remove(path, chunk_type);
        match result {
            Ok(_) => Ok(()),
            Err(e) => match e {
                Error::FileNotFound { source: _, path: _ } => {
                    Err(PyFileNotFoundError::new_err(e.to_string()))
                }
                Error::Read { source: _ } => Err(PyIOError::new_err(e.to_string())),
                Error::PNGParse => Err(PyValueError::new_err(e.to_string())),
                Error::InvalidChunkType {
                    chunk_type: _,
                    source: s,
                } => Err(PyValueError::new_err(s.to_string())),
                Error::PNGWrite { source: _ } => Err(PyValueError::new_err(e.to_string())),
                Error::ChunkNotFound { chunk_type: _ } => Err(PyValueError::new_err(e.to_string())),
                Error::StrConversion => Err(PyValueError::new_err(e.to_string())),
            },
        }
    }
}
