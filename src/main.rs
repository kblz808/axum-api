use axum::{routing::get, Router};

// async fn hello_world() -> &'static str {
//     "hello world"
// }

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/", get(hello_world));

    let app = Router::new().route("/", get(|| async { "hello again" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
