from __future__ import annotations

from typing import Any


def annotation_kind(annotation: Any) -> str:
    if annotation is str:
        return "str"
    if annotation is int:
        return "int"
    if annotation is float:
        return "float"
    if annotation is bool:
        return "bool"
    if annotation is None or annotation is type(None):
        return "none"
    if is_body_model(annotation):
        return "pydantic"
    return "any"


def is_body_model(annotation: Any) -> bool:
    try:
        from pydantic import BaseModel
    except ImportError:
        return False

    return isinstance(annotation, type) and issubclass(annotation, BaseModel)
