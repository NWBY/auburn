mod errors;
mod extractors;
mod request;
mod response;
mod router;
mod server;

use pyo3::prelude::*;

#[pyfunction]
fn serve(py: Python<'_>, routes: Bound<'_, PyAny>, host: String, port: u16) -> PyResult<()> {
    server::serve(py, routes, host, port)
}

#[pymodule]
fn auburn_core(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(serve, module)?)?;
    Ok(())
}
