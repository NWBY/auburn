use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, patch, post, put, MethodRouter};
use axum::Router;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::response;

pub struct RoutePlan {
    pub method: String,
    pub path: String,
    pub handler: Py<PyAny>,
}

pub fn plans_from_python(routes: Bound<'_, PyAny>) -> PyResult<Vec<RoutePlan>> {
    let route_list = routes.downcast::<PyList>()?;
    let mut plans = Vec::with_capacity(route_list.len());

    for item in route_list.iter() {
        let route = item.downcast::<PyDict>()?;
        let method: String = route
            .get_item("method")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("route missing method"))?
            .extract()?;
        let path: String = route
            .get_item("path")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("route missing path"))?
            .extract()?;
        let handler: Py<PyAny> = route
            .get_item("handler")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("route missing handler"))?
            .unbind();

        plans.push(RoutePlan {
            method,
            path,
            handler,
        });
    }

    Ok(plans)
}

pub fn build_router(plans: Vec<RoutePlan>) -> PyResult<Router> {
    let mut router = Router::new();

    for plan in plans {
        let path = plan.path.clone();
        let method_router = method_router(Arc::new(plan))?;
        router = router.route(&path, method_router);
    }

    Ok(router)
}

fn method_router(plan: Arc<RoutePlan>) -> PyResult<MethodRouter> {
    let method = plan.method.as_str();

    let route = match method {
        "GET" => get(move || handle(plan)),
        "POST" => post(move || handle(plan)),
        "PUT" => put(move || handle(plan)),
        "PATCH" => patch(move || handle(plan)),
        "DELETE" => delete(move || handle(plan)),
        other => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unsupported HTTP method {other}"
            )))
        }
    };

    Ok(route)
}

async fn handle(plan: Arc<RoutePlan>) -> Response {
    Python::with_gil(|py| match plan.handler.call0(py) {
        Ok(value) => match response::json_from_python(value.bind(py)) {
            Ok(body) => {
                (StatusCode::OK, [("content-type", "application/json")], body).into_response()
            }
            Err(err) => response::python_error_response(err),
        },
        Err(err) => response::python_error_response(err),
    })
}
