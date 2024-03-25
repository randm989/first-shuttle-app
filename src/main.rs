use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rocket::serde::json::json;
use serde::Serialize;
use sqlx::PgPool;

// here we show a type that implements Serialize + Send
#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}

enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::Unauthorised => (StatusCode::UNAUTHORIZED).into_response(),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

async fn hello_world() -> &'static str {
    "Hello, Postgres world!"
}

async fn hello_state() -> Result<ApiResponse, ApiError> {
    //Ok(ApiResponse::OK)
    /*Json(Message {
        message: format!("Incoming payload was: {}", param.message),
    })*/
    let data = vec![Message {
        message: String::from("Hello messaged world"),
    }];
    Ok(ApiResponse::JsonData(data))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    //async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    //let state = MyState { pool };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/on_state", get(hello_state));

    Ok(router.into())
}
