use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateContact {
    pub name: String,
    pub email: String,
    pub message: String,
    pub interest: String,
}
