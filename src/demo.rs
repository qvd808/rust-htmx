use std::net::SocketAddr;
use axum::{response::Html, Router, routing::get};
use lazy_static::lazy_static;
use tera::Tera;
use tokio::runtime::Handle;
use futures;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

async fn root_handler() -> Html<String>{

    let r = TEMPLATES.render("base.html", &tera::Context::new()).unwrap();
    Html(r)
}

#[tokio::main]
pub async fn demo (handle: Handle) {

    futures::executor::block_on(async  {
        handle.spawn(
            async {

            }
        ).await.expect("Task spawned in Tokio executor panicked!")
    });
    
    let route_hello = Router::new().route(
        "/",
        get(root_handler),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(route_hello.into_make_service())
        .await
        .unwrap();
}