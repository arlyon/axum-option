//! Run with
//!
//! ```not_rust
//! cargo run --example query
//! ```

use std::net::SocketAddr;

use axum::{extract::Query, response::Html, routing::get, Router};
use axum_option::ValidOption;
use serde::Deserialize;

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

#[derive(Deserialize, Clone, Debug)]
struct SearchQuery {
    search: String,
}

// in this case, we want to reject requests that don't have a valid
// Query, so we use `ValidOption` to do that
#[axum_macros::debug_handler]
async fn handler(ValidOption(query): ValidOption<Query<SearchQuery>>) -> Html<String> {
    Html(format!(
        "<h1>Searching for {}!</h1>",
        query
            .as_ref()
            .map(|a| a.0.search.as_ref())
            .unwrap_or("nothing")
    ))
}

// in the incorrect case, if you send an invalid Query
// it will be ignored and the request will be treated as if it didn't
// have an Query at all
#[axum_macros::debug_handler]
async fn incorrect(query: Option<Query<SearchQuery>>) -> Html<String> {
    Html(format!(
        "<h1>Searching for {}!</h1>",
        query
            .as_ref()
            .map(|a| a.0.search.as_ref())
            .unwrap_or("nothing")
    ))
}
