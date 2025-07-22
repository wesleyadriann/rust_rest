use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::models::contact::CreateContact;
use crate::models::response::ResponseBody;

pub async fn get_contacts() -> Json<ResponseBody> {
    println!("get_contacts - start");
    let response_body = ResponseBody {
        message: "List of contacts".to_string(),
    };

    println!("get_contacts - end");
    Json(response_body)
}

pub async fn create_contact(Json(payload): Json<CreateContact>) -> impl IntoResponse {
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
