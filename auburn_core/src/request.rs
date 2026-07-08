use pyo3::prelude::*;
use pyo3::types::PyDict;

pub struct RoutePlan {
    pub method: String,
    pub path: String,
    pub handler: Py<PyAny>,
    pub params: Vec<ParamPlan>,
}

pub struct ParamPlan {
    pub name: String,
    pub source: ParamSource,
    pub kind: ParamKind,
    pub required: bool,
}
pub enum ParamSource {
    Path,
    Query,
    Body,
}

pub enum ParamKind {
    Str,
    Int,
    Float,
    Bool,
    Pydantic,
    Any,
}

impl ParamPlan {
    pub fn from_python(param: &Bound<'_, PyDict>) -> PyResult<Self> {
        let name: String = param
            .get_item("name")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("param missing name"))?
            .extract()?;

        let source: String = param
            .get_item("source")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("param missing source"))?
            .extract()?;

        let kind: String = param
            .get_item("kind")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("param missing kind"))?
            .extract()?;

        let required: bool = param
            .get_item("required")?
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("param missing required"))?
            .extract()?;

        Ok(Self {
            name,
            source: ParamSource::from_str(&source)?,
            kind: ParamKind::from_str(&kind)?,
            required,
        })
    }
}

impl ParamSource {
    pub fn from_str(value: &str) -> PyResult<Self> {
        match value {
            "path" => Ok(Self::Path),
            "query" => Ok(Self::Query),
            "body" => Ok(Self::Body),
            other => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unsupported param source {other}"
            ))),
        }
    }
}

impl ParamKind {
    pub fn from_str(value: &str) -> PyResult<Self> {
        match value {
            "str" => Ok(Self::Str),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "bool" => Ok(Self::Bool),
            "pydantic" => Ok(Self::Pydantic),
            "any" => Ok(Self::Any),
            other => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unsupported param kind {other}"
            ))),
        }
    }
}
