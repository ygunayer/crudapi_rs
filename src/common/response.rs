use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    data: T
}

pub fn response_of<T>(data: T) -> Json<Response<T>> {
    let response = Response { data };
    Json(response)
}
