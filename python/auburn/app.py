from __future__ import annotations

import inspect
from collections.abc import Callable
from typing import Any, get_type_hints

from .routing import ParamPlan, RoutePlan
from .schemas import annotation_kind, is_body_model

Handler = Callable[..., Any]


class App:
    def __init__(self) -> None:
        self._routes: list[RoutePlan] = []

    @property
    def routes(self) -> tuple[RoutePlan, ...]:
        return tuple(self._routes)

    def get(self, path: str) -> Callable[[Handler], Handler]:
        return self.route("GET", path)

    def post(self, path: str) -> Callable[[Handler], Handler]:
        return self.route("POST", path)

    def put(self, path: str) -> Callable[[Handler], Handler]:
        return self.route("PUT", path)

    def patch(self, path: str) -> Callable[[Handler], Handler]:
        return self.route("PATCH", path)

    def delete(self, path: str) -> Callable[[Handler], Handler]:
        return self.route("DELETE", path)

    def route(self, method: str, path: str) -> Callable[[Handler], Handler]:
        def decorator(handler: Handler) -> Handler:
            self.add_route(method, path, handler)
            return handler

        return decorator

    def add_route(self, method: str, path: str, handler: Handler) -> None:
        signature = inspect.signature(handler)
        type_hints = get_type_hints(handler)
        path_param_names = _path_param_names(path)
        params: list[ParamPlan] = []

        for name, parameter in signature.parameters.items():
            annotation = type_hints.get(name, parameter.annotation)
            source = _param_source(name, annotation, path_param_names)
            params.append(
                ParamPlan(
                    name=name,
                    source=source,
                    kind=annotation_kind(annotation),
                    required=parameter.default is inspect.Signature.empty,
                )
            )

        self._routes.append(
            RoutePlan(
                method=method.upper(),
                path=path,
                handler=handler,
                params=tuple(params),
            )
        )

    def core_routes(self) -> list[dict[str, Any]]:
        return [route.to_core_dict() for route in self._routes]

    def run(self, host: str = "127.0.0.1", port: int = 8000) -> None:
        try:
            import auburn_core
        except ImportError as exc:
            raise RuntimeError(
                "auburn_core is not installed. Run `uv run maturin develop` before "
                "starting the Rust server."
            ) from exc

        auburn_core.serve(self.core_routes(), host, port)


def _path_param_names(path: str) -> set[str]:
    names: set[str] = set()
    for part in path.split("/"):
        if part.startswith("{") and part.endswith("}"):
            names.add(part[1:-1])
    return names


def _param_source(name: str, annotation: Any, path_param_names: set[str]) -> str:
    if name in path_param_names:
        return "path"
    if is_body_model(annotation):
        return "body"
    return "query"
