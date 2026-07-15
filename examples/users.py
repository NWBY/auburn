from .health import app


@app.get("/users/{user_id}")
def get_user(user_id: int):
    return {"id": user_id}
