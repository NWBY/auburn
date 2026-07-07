from auburn import App

app = App()


@app.get("/health")
def health():
    return {"ok": True}
