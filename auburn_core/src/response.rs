use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use pyo3::prelude::*;
use pyo3::types::PyString;

pub fn json_from_python(value: &Bound<'_, PyAny>) -> PyResult<String> {
    let py = value.py();
    let json = py.import_bound("json")?;
    let dumps = json.getattr("dumps")?;
    let rendered = dumps.call1((value,))?;
    let rendered = rendered.downcast::<PyString>()?;
    rendered.to_str().map(str::to_owned)
}

pub fn python_error_response(err: PyErr) -> Response {
    let message = err.to_string();
    let body = serde_json::json!({
        "error": "internal_server_error",
        "detail": message,
    })
    .to_string();

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [("content-type", "application/json")],
        body,
    )
        .into_response()
}
