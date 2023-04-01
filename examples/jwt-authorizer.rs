//! Run with
//!
//! ```not_rust
//! cargo run --example jwt-authorizer
//! ```

use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};
use axum_option::ValidOption;
use jwt_authorizer::{layer::JwtSource, JwtAuthorizer, JwtClaims};
use serde::Deserialize;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let auth: JwtAuthorizer<()> = jwt_authorizer::JwtAuthorizer::from_secret("secret");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/incorrect", get(incorrect))
        .layer(
            auth.jwt_source(JwtSource::Cookie("jwt".to_string()))
                .layer()
                .await
                .unwrap(),
        );

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Clone)]
struct User {
    username: String,
    exp: usize,
}

type Claims = JwtClaims<User>;

// in this case, we want to reject requests that don't have a valid
// jwt token, so we use `ValidOption` to do that
async fn handler(ValidOption(query): ValidOption<Claims>) -> Html<String> {
    Html(format!(
        "<h1>Hello, {}!</h1>",
        query
            .as_ref()
            .map(|a| a.0.username.as_str())
            .unwrap_or("unknown")
    ))
}

// in the incorrect case, if you have an invalid jwt token
// it will be ignored and the request will be treated as if it didn't
// have a token attached at all
async fn incorrect(query: Option<Claims>) -> Html<String> {
    Html(format!(
        "<h1>Hello, {}!</h1>",
        query
            .as_ref()
            .map(|a| a.0.username.as_str())
            .unwrap_or("unknown")
    ))
}
