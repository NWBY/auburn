from __future__ import annotations

from collections.abc import Callable
from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class ParamPlan:
    name: str
    source: str
    kind: str
    required: bool = True

    def to_core_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "source": self.source,
            "kind": self.kind,
            "required": self.required,
        }


@dataclass(frozen=True)
class RoutePlan:
    method: str
    path: str
    handler: Callable[..., Any]
    params: tuple[ParamPlan, ...]

    def to_core_dict(self) -> dict[str, Any]:
        return {
            "method": self.method,
            "path": self.path,
            "handler": self.handler,
            "params": [param.to_core_dict() for param in self.params],
        }
