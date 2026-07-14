use pyo3::prelude::*;
use pyo3::types::PyDict;

#[allow(dead_code)]
pub struct RoutePlan {
    pub method: String,
    pub path: String,
    pub handler: Py<PyAny>,
    pub params: Vec<ParamPlan>,
}

#[allow(dead_code)]
pub struct ParamPlan {
    pub name: String,
    pub source: ParamSource,
    pub kind: ParamKind,
    pub required: bool,
}

#[allow(dead_code)]
pub enum ParamSource {
    Path,
    Query,
    Body,
}

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::{ParamKind, ParamSource};

    #[test]
    fn parses_supported_param_sources() {
        assert!(matches!(
            ParamSource::from_str("path"),
            Ok(ParamSource::Path)
        ));
        assert!(matches!(
            ParamSource::from_str("query"),
            Ok(ParamSource::Query)
        ));
        assert!(matches!(
            ParamSource::from_str("body"),
            Ok(ParamSource::Body)
        ));
        assert!(ParamSource::from_str("header").is_err());
    }

    #[test]
    fn parses_supported_param_kinds() {
        assert!(matches!(ParamKind::from_str("str"), Ok(ParamKind::Str)));
        assert!(matches!(ParamKind::from_str("int"), Ok(ParamKind::Int)));
        assert!(matches!(ParamKind::from_str("float"), Ok(ParamKind::Float)));
        assert!(matches!(ParamKind::from_str("bool"), Ok(ParamKind::Bool)));
        assert!(matches!(
            ParamKind::from_str("pydantic"),
            Ok(ParamKind::Pydantic)
        ));
        assert!(matches!(ParamKind::from_str("any"), Ok(ParamKind::Any)));
        assert!(ParamKind::from_str("uuid").is_err());
    }
}
