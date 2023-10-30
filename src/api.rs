use gloo_net::http::Request;
use serde::Deserialize;

use crate::models::UserOrError;

#[derive(Clone, PartialEq, Deserialize)]
pub struct UserResponse {
    pub ok: bool,
    pub data: UserOrError,
}

pub async fn user_from_session() -> UserResponse {
    let response: UserResponse = Request::get("/api/users/user_from_session")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    response
}
