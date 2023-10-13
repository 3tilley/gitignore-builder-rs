use axum::{routing::get, Router};
use gitignore_builder_rs::make_router;

#[tokio::main]
async fn main() {
    let router =make_router();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
