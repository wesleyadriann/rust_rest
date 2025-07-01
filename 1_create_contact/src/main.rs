use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateContact {
    name: String,
    email: String,
    message: String,
    interest: String,
}

#[derive(Serialize)]
struct ResponseBody {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_contacts).post(create_contact));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_contacts() -> Json<ResponseBody> {
    println!("get_contacts - start");
    let response_body = ResponseBody {
        message: "List of contacts".to_string(),
    };

    println!("get_contacts - end");
    Json(response_body)
}

async fn create_contact(Json(payload): Json<CreateContact>) -> impl IntoResponse {
    println!(
        "create_contact - start - body = [name = {}, email = {}, message = {}, interest = {}]",
        payload.name, payload.email, payload.message, payload.interest
    );

    let contact_name = payload.name;
    let response_body = ResponseBody {
        message: format!(
            "Thank {}, for getting in touch and sharing your interests. We look forward to hearing from you soon.",
            contact_name
        ),
    };

    println!("create_contact - end");
    (StatusCode::CREATED, Json(response_body))
}
