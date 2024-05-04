extern crate axum;
extern crate tokio;

use std::collections::HashMap;

use axum::extract::{Query, Request};
use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/greeting", get(greeting));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:18888")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn greeting(Query(params): Query<HashMap<String, String>>, request: Request) -> Html<String> {
    println!("{:#?}", request);
    let response = format!(
        "<html><body>Hello, {}!</body></html>",
        params.get("name").unwrap()
    );
    Html(response)
}
