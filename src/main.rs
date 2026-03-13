use std::path::Path;
use axum::Router;
use http::{HeaderName, HeaderValue};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_header::SetResponseHeaderLayer;
// Warning - HMR does not work with Axum.

#[tokio::main(flavor = "multi_thread", worker_threads = 25)]
async fn main() {
    let assets = Path::new("assets");
    let tcp = TcpListener::bind(("0.0.0.0", 80)).await.expect("Failed to bind TcpListener on port 80. Perhaps blocked by the firewall?");

    // Add other origins you use here.
    let cors = CorsLayer::default();



    let router = Router::<()>::new()
        .route_service("/", ServeFile::new(assets.join("index.html")))
        .nest_service("/assets", ServeDir::new(assets.join("assets")))
        .route_service("/favicon.svg", ServeFile::new(assets.join("favicon.svg")))
        .route_service("/icons.svg", ServeDir::new(assets.join("icons.svg")))
        .layer(cors)
        .layer(SetResponseHeaderLayer::if_not_present(
            http::header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static(
                "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'",
            ),
        ))
        // April fools joke :D
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-powered-by"),
            HeaderValue::from_static("Django/5.0"),
        ));
    axum::serve(tcp, router).await.expect("Could not start web app.");
}
