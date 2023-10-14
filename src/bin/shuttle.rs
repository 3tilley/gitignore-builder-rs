use gitignore_builder_rs::make_router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = make_router();
    Ok(router.into())
}
