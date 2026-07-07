use pyo3::prelude::*;
use tokio::net::TcpListener;

use crate::router;

pub fn serve(py: Python<'_>, routes: Bound<'_, PyAny>, host: String, port: u16) -> PyResult<()> {
    let plans = router::plans_from_python(routes)?;
    let address = format!("{host}:{port}");
    let app = router::build_router(plans)?;

    let runtime = tokio::runtime::Runtime::new()
        .map_err(|err| pyo3::exceptions::PyRuntimeError::new_err(err.to_string()))?;

    py.allow_threads(|| {
        runtime.block_on(async move {
            let listener = TcpListener::bind(&address)
                .await
                .map_err(|err| pyo3::exceptions::PyRuntimeError::new_err(err.to_string()))?;
            axum::serve(listener, app)
                .await
                .map_err(|err| pyo3::exceptions::PyRuntimeError::new_err(err.to_string()))
        })
    })
}
