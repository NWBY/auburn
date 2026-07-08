use pyo3::prelude::*;

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
