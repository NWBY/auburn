from auburn.cli import load_app

from auburn import App


def test_get_decorator_registers_route_plan():
    app = App()

    @app.get("/health")
    def health():
        return {"ok": True}

    route = app.routes[0]

    assert route.method == "GET"
    assert route.path == "/health"
    assert route.handler is health
    assert route.params == ()


def test_path_parameter_is_marked_as_path_source():
    app = App()

    @app.get("/users/{user_id}")
    def get_user(user_id: int):
        return {"id": user_id}

    route = app.routes[0]
    param = route.params[0]

    assert param.name == "user_id"
    assert param.source == "path"
    assert param.kind == "int"


def test_load_app_resolves_example_app():
    app = load_app("examples.health:app")

    assert isinstance(app, App)
    assert app.routes[0].path == "/health"
