use pyo3::{PyAny, PyResult};
use pyo3::exceptions::PyValueError;

pub fn check_callable(callback: &PyAny) -> PyResult<()> {
    if !callback.is_callable() {
        return Err(PyValueError::new_err("Parameter passed into transform is not callable."));
    }
    Ok(())
}