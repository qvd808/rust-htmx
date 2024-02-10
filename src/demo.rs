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
    id: i32,
    name: String,
    description: String,
}

async fn items_handler() -> Html<String> {
    let mut context = tera::Context::new();

    let mut item_list: Vec<Item> = Vec::new();
    let ipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla nec purus feugiat, molestie ipsum et, consequat nibh. Etiam non elit dui. Nullam vel eros sit amet arcu vestibulum accumsan in in leo.";
    for i in 0..10 {
        let item = Item {
            id: i,
            name: format!("Item {}", i),
            description: format!("Description of item {}", ipsum),
        };
        item_list.push(item);
    }
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
        .route("/item", get(items_handler))
        .nest("/static", axum_static::static_router("templates/assets"))
        .route(
            "/modal-add-item",
            get(|| async {
                let r = TEMPLATES
                    .render("modal/addItem.html", &tera::Context::new())
                    .unwrap();
                Html(r)
            }),
        )
        .route(
            "/modal-edit-item",
            get(|| async {
                let r = TEMPLATES
                    .render("modal/addButton.html", &tera::Context::new())
                    .unwrap();
                Html(r)
            }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(route_hello.into_make_service())
        .await
        .unwrap();
}
