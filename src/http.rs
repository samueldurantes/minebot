use std::env;

use axum::{routing::get, Router};

pub async fn http_server() {
  let app = Router::new().route("/healthz", get(|| async { "ok!" }));

  let addr = format!("0.0.0.0:{}", env::var("HTTP_PORT").unwrap_or("3000".to_string()));
  let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();

  println!("HTTP server started on {}", addr);
  axum::serve(listener, app).await.unwrap();
}
