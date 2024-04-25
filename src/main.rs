use axum::extract::Request;
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

async fn greeting(request: Request) -> Html<&'static str> {
    println!("{:#?}", request);
    Html("<html><body>Hello, World!</body></html>")
}
