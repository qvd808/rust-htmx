use axum::{
    body::Body,
    extract::{Form, Path},
    response::{Html, Response},
    routing::{get, post, put},
    Router,
};
use lazy_static::lazy_static;
use std::net::SocketAddr;
use tera::Tera;

pub mod database;
pub mod item;
use database::Database;
use item::Item;

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

async fn get_single_item_handler() -> Html<String> {
    let db = Database::new();
    let item_list = db.get_all_items();
    let mut context = tera::Context::new();

    match item_list {
        Ok(item_list) => {
            context.insert("items", &item_list);
            let r = TEMPLATES.render("items.html", &context).unwrap();
            Html(r)
        }
        Err(_) => {
            println!("Error getting items");
            Html("Error getting items".to_string())
        }
    }


}

async fn put_single_item_handler(Form(params): Form<Item>) ->Response<Body> {
    let db = Database::new();

    let res = db.update_item(params);
    // let res:Result<(), anyhow::Error> = Ok(() );

    match res {
        Ok(_) => 
        Response::builder()
            .status(200)
            .header("Content-Type", "text/html")
            .header("HX-Refresh", "true")
            .body(Body::from("Item updated"))
            .unwrap(),
        Err(_) => Response::builder()
            .status(500)
            .header("Content-Type", "text/html")
            .body(Body::from("Error adding item"))
            .unwrap(),
    }
}

async fn update_single_item_handler(Path(id): Path<i32>) -> Html<String> {
    let mut context = tera::Context::new();

    let db = Database::new();
    let item = db.get_item_with_id(id as i64);
    match item {
        Some(item) => {
            context.insert("name", &item.get_name());
            context.insert("description", &item.get_description());
            context.insert("method", "hx-put=/api/item");
            context.insert("id", &item.id);
            let r = TEMPLATES.render("modal/addItem.html", &context).unwrap();
            Html(r)
        }
        None => {
            //     context.insert("id", &0);
            //     context.insert("name", &"");
            //     context.insert("description", &"");
            // }
            context.insert("error", "The item with the given id does not exist. Please try again.");

            let r = TEMPLATES.render("modal/error.html", &context).unwrap();
            Html(r)
        }
    }


}

async fn add_item_handler(Form(params): Form<Item>) -> Response<Body> {
    let insert_item = Item::new(None, params.get_name(), params.get_description());

    let db = Database::new();
    let res = db.add_item(insert_item);

    match res {
        Ok(_) => Response::builder()
            .status(200)
            .header("Content-Type", "text/html")
            .header("HX-Refresh", "true")
            .body(Body::from("Item added"))
            .unwrap(),
        Err(_) => Response::builder()
            .status(500)
            .header("Content-Type", "text/html")
            .body(Body::from("Error adding item"))
            .unwrap(),
    }
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
        .route("/api/item", get(get_single_item_handler))
        .route("/api/add-item", post(add_item_handler))
        .nest("/static", axum_static::static_router("templates/assets"))
        .route(
            "/modal-add-item",
            get(|| async {
                let mut context = tera::Context::new();
                context.insert("name", "");
                context.insert("description", "");
                context.insert("method", "hx-post=/api/add-item");
                context.insert("id", &0);

                let r = TEMPLATES.render("modal/addItem.html", &context).unwrap();
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
        )
        .route("/modal-item/:id", get(update_single_item_handler))
        .route("/api/item", put(put_single_item_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(route_hello.into_make_service())
        .await
        .unwrap();
}
