use axum::{response::Html, routing::get, Router};
use futures;
use lazy_static::lazy_static;
use serde::Serialize;
use std::net::SocketAddr;
use tera::Tera;
use tokio::runtime::Handle;

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

#[derive(Serialize)]
struct Item {
    name: String,
    description: String,
}

async fn items_handler() -> Html<String> {
    let mut context = tera::Context::new();
    let item_list: [Item; 2] = [
        Item {
            name: "Item 1".to_string(),
            description: "Description of item 1".to_string(),
        },
        Item {
            name: "Item 2".to_string(),
            description: "Description of item 2".to_string(),
        },
    ];
    context.insert("items", &item_list);
    let r = TEMPLATES.render("items.html", &context).unwrap();
    Html(r)
}

async fn root_handler() -> Html<String> {
    let r = TEMPLATES
        .render("base.html", &tera::Context::new())
        .unwrap();
    Html(r)
}

#[tokio::main]
pub async fn demo(handle: Handle) {
    futures::executor::block_on(async {
        handle
            .spawn(async {})
            .await
            .expect("Task spawned in Tokio executor panicked!")
    });

    let route_hello = Router::new()
        .route("/", get(root_handler))
        .route("/item", get(items_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(route_hello.into_make_service())
        .await
        .unwrap();
}
