use axum::{body::Body, extract::{Query, Form}, response::{Html, Response}, routing::{get, post}, Router};
use futures;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
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


#[derive(Serialize, Clone, Deserialize, Debug)]
struct Item {
    name: String,
    description: String,
}

use std::sync::Mutex;

lazy_static! {
    static ref ITEMS: Mutex<Vec<Item>> = Mutex::new(Vec::new());
}

async fn items_handler() -> Html<String> {
    let mut context = tera::Context::new();

    let item_list: Vec<Item> = ITEMS.lock().unwrap().clone();

    context.insert("items", &item_list);
    let r = TEMPLATES.render("items.html", &context).unwrap();
    Html(r)
}



async fn add_item_handler(Form(params): Form<Item>) -> Response<Body>{

    // let lens = ITEMS.lock().unwrap().len() + 1;
    println!("Adding item {:?}", params);


    ITEMS.lock().unwrap().push(Item {
        name: format!("Item {}", params.name),
        description: format!("Description {}", params.description),
    });
      
    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .header("HX-Refresh", "true")
        .body(Body::from("not found"))
        .unwrap()
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
        .route("/api/item", get(items_handler))
        .route("/api/add-item", post(add_item_handler))
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
