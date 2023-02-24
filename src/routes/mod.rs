mod hello_world;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use hello_world::hello_world;

pub fn create_routes() -> Router<Body> {
    // Router::new().route("/", get(hello_world))
    Router::new().route("/", post(hello_world))
}
