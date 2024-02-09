// #![allow(unused)]

// use std::net::SocketAddr;

// use axum::{response::Html, Router, routing::get};

// #[tokio::main]
// async fn main() {
//     let route_hello = Router::new().route(
//         "/hello",
//         get(|| async { Html("Hello <strong> World!! </strong>")}),
//     );

//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     println!("Listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(route_hello.into_make_service())
//         .await
//         .unwrap();
// }

pub mod demo;
use demo::demo;

use tokio::runtime::Handle;

#[tokio::main]
async fn main() {
    let handle = Handle::current();
    tokio::task::spawn_blocking(|| {
        demo(handle);
    })
    .await
    .expect("Demo failed to run")
}