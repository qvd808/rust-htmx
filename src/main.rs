use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
    extract::{Path, Query},
};

use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive( Deserialize)]
struct Page {
    number: u32,
}

//Handle for /create-user
async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

//Handle  for /users
async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string()
        }

    ];
    Json(users)
}

async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String{
    format!("Item {} on page {}\n", id, page.number)
}


#[tokio::main]
async fn main() {
    const API: &str = "http://localhost:3000";

    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users))
        .route("/item/:id", get(show_item));

    println!("Running on {}", API);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
