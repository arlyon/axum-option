//! Run with
//!
//! ```not_rust
//! cargo run --example path
//! ```

use std::net::SocketAddr;

use axum::{extract::Path, response::Html, routing::get, Router};
use axum_option::ValidOption;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/:id", get(handler))
        .route("/incorrect/:id", get(incorrect));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// in this case, we want to reject requests that don't have a valid
// Path, so we use `ValidOption` to do that
#[axum_macros::debug_handler]
async fn handler(ValidOption(query): ValidOption<Path<u32>>) -> Html<String> {
    Html(format!(
        "<h1>Viwing {}!</h1>",
        query
            .as_ref()
            .map(|a| format!("id {}", a.0))
            .unwrap_or("all".to_string())
    ))
}

// in the incorrect case, if you send an invalid Path
// it will be ignored and the request will be treated as if it didn't
// have an Authorization header at all
#[axum_macros::debug_handler]
async fn incorrect(query: Option<Path<u32>>) -> Html<String> {
    Html(format!(
        "<h1>Viwing {}!</h1>",
        query
            .as_ref()
            .map(|a| format!("id {}", a.0))
            .unwrap_or("all".to_string())
    ))
}
