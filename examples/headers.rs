//! Run with
//!
//! ```not_rust
//! cargo run --example headers
//! ```

use std::net::SocketAddr;

use axum::{
    headers::{authorization::Basic, Authorization},
    response::Html,
    routing::get,
    Router, TypedHeader,
};
use axum_option::ValidOption;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/incorrect", get(incorrect));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type BasicHeader = TypedHeader<Authorization<Basic>>;

// in this case, we want to reject requests that don't have a valid
// Authorization header, so we use `ValidOption` to do that
async fn handler(ValidOption(query): ValidOption<BasicHeader>) -> Html<String> {
    Html(format!(
        "<h1>Hello, {}!</h1>",
        query
            .as_ref()
            .map(|a| a.0 .0.username())
            .unwrap_or("unknown")
    ))
}

// in the incorrect case, if you send an invalid Authorization header
// it will be ignored and the request will be treated as if it didn't
// have an Authorization header at all
async fn incorrect(query: Option<BasicHeader>) -> Html<String> {
    Html(format!(
        "<h1>Hello, {}!</h1>",
        query
            .as_ref()
            .map(|a| a.0 .0.username())
            .unwrap_or("unknown")
    ))
}
