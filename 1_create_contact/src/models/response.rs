use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseBody {
    pub message: String,
}
