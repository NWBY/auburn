use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, patch, post, put, MethodRouter};
use axum::Router;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::request::{ParamPlan, RoutePlan};
use crate::response;

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
        let params_list = route
            .get_item("params")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("route missing params"))?
            .downcast_into::<PyList>()?;

        let mut params = Vec::with_capacity(params_list.len());

        for item in params_list.iter() {
            let param = item.downcast::<PyDict>()?;
            params.push(ParamPlan::from_python(param)?);
        }

        plans.push(RoutePlan {
            method,
            path,
            handler,
            params,
        });
    }

    Ok(plans)
}

pub fn build_router(plans: Vec<RoutePlan>) -> PyResult<Router> {
    let mut router = Router::new();

    for plan in plans {
        let path = axum_path(&plan.path);
        let method_router = method_router(Arc::new(plan))?;
        router = router.route(&path, method_router);
    }

    Ok(router)
}

fn axum_path(path: &str) -> String {
    path.split("/")
        .map(|segment| {
            if let Some(name) = segment
                .strip_prefix("{")
                .and_then(|segment| segment.strip_suffix("}"))
            {
                format!(":{name}")
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
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

#[cfg(test)]
mod tests {
    use super::axum_path;

    #[test]
    fn translates_auburn_path_parameters_for_axum() {
        assert_eq!(axum_path("/health"), "/health");
        assert_eq!(axum_path("/users/{user_id}"), "/users/:user_id");
        assert_eq!(
            axum_path("/users/{user_id}/posts/{post_id}"),
            "/users/:user_id/posts/:post_id",
        );
    }
}
