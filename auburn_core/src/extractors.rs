// Path, query, and JSON body extractors live here.
use std::collections::HashMap;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::request::{ParamKind, ParamPlan, ParamSource};

pub fn path_kwargs<'py>(
    py: Python<'py>,
    plans: &[ParamPlan],
    path_params: &HashMap<String, String>,
) -> PyResult<Bound<'py, PyDict>> {
}
