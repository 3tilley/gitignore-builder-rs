mod stuff;

use axum::Router;
use axum::routing::get;

pub async fn hello_world() -> &'static str {
    "Hello from a non-standard package layout"
}

pub fn make_router() -> Router {
    Router::new().route("/", get(hello_world))
}
