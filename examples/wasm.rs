extern crate tokio;
extern crate warp;

use warp::Filter;

#[tokio::main]
async fn main() {
    std::process::Command::new("wasm-pack")
        .args(&["build", "--target", "web"])
        .output()
        .expect("wasm-pack failed -- try `cargo install wasm-pack`");

    let routes = warp::get().and(warp::path::end()).and(warp::fs::file("examples/wasm.html"))
        .or(warp::fs::dir("pkg"));

    let serve_result = warp::serve(routes).run(([127, 0, 0, 1], 4353));

    println!("Check out this wasm example at https://127.0.0.1:4353");

    serve_result.await;
}
