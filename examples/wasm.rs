extern crate tokio;
#[macro_use] extern crate warp;

use tokio_compat_02::FutureExt;
use warp::{Filter, http::header::{HeaderMap, HeaderValue}};

#[tokio::main]
async fn main() {
    std::process::Command::new("wasm-pack")
        .args(&["build", "--dev", "--target", "web"])
        .output()
        .expect("wasm-pack failed -- try `cargo install wasm-pack`");

    let mut headers = HeaderMap::new();
    headers.insert("Cross-Origin-Opener-Policy", HeaderValue::from_static("same-origin"));
    headers.insert("Cross-Origin-Embedder-Policy", HeaderValue::from_static("require-corp"));
    
    let routes = warp::get().and(warp::path::end())
            .and(warp::fs::file("examples/wasm.html"))
        .or(warp::get().and(path!("worker.js")).and(warp::path::end())
            .and(warp::fs::file("examples/worker.js").with(warp::reply::with::headers(headers.clone()))))
        .or(warp::fs::dir("pkg").with(warp::reply::with::headers(headers)));

    let serve_result = warp::serve(routes)
        .tls()
        .cert_path("examples/cert.pem")
        .key_path("examples/key.rsa")
        .run(([127, 0, 0, 1], 4353));

    println!("Check out this wasm example at http://127.0.0.1:4353");

    serve_result
        .compat()
        .await;
}
