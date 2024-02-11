
use axum::{body::Body, extract::Form, response::{Html, Response}, routing::{get, post}, Router};
use lazy_static::lazy_static;
use std::net::SocketAddr;
use tera::Tera;

pub mod item;
pub mod database;
use item::Item;
use database::Database;

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


async fn items_handler() -> Html<String> {

    let db = Database::new();
    let item_list = db.get_all_items(); 
    let mut context = tera::Context::new();

    for item in &item_list {
        match item.id {
            Some(id) => {
                println!("id: {}", id);
                println!("name: {}", item.get_name());
                println!("description: {}", item.get_description());
            }
            None => {
                println!("name: {}", item.get_name());
                println!("description: {}", item.get_description());
            }
        }
    }

    context.insert("items", &item_list);
    let r = TEMPLATES.render("items.html", &context).unwrap();
    Html(r)
}



async fn add_item_handler(Form(params): Form<Item>) -> Response<Body>{

    let insert_item = Item::new(
        None,
        params.get_name(),
        params.get_description()
    );

    let db = Database::new();
    db.add_item(insert_item);

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
pub async fn main() {
    let db = Database::new();
    db.create_table();

    let route_hello = Router::new()
        .route("/", get(root_handler))
        .route("/api/item", get(items_handler))
        .route("/api/add-item", post(add_item_handler) )
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
