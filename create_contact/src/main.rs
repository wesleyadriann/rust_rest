use axum::{Router, routing::get};

use crate::controllers::contact_controller::{create_contact, get_contacts};

pub mod controllers;
pub mod db;
pub mod models;

#[tokio::main]
async fn main() {
    let db_connection = db::create_connection();
    let app = Router::new().route("/", get(get_contacts).post(create_contact));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
