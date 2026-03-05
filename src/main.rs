use axum::{Router, body::Bytes, routing::post};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/echo", post(echo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn echo(body: Bytes) -> Bytes {
    body
}
