use gitignore_builder_rs::{make_router, prepare_tracing};

#[tokio::main]
async fn main() {
    prepare_tracing();
    let router = make_router();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
