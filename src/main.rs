use axum::{
    extract::State,
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
//use rocket::serde::json::json;
use ramhorns::{Content, Ramhorns, Template};
use serde::Serialize;
use sqlx::{FromRow, PgPool, Row};
use tracing::info;

// here we show a type that implements Serialize + Send
#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

#[derive(Debug, FromRow)]
struct Person {
    name: String,
    number: i32,
}

#[derive(Content, Debug)]
struct PersonContent {
    title: String,
    number_data: i32,
}

enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
    HTML(String),
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
            Self::HTML(data) => (StatusCode::OK, Html(data)).into_response(),
        }
    }
}

async fn hello_world() -> Result<ApiResponse, ApiError> {
    //&'static str {
    let source = "<h1>{{title}}</h1>\
{{number_data}}\
";
    let person = PersonContent {
        title: "TestName".into(),
        number_data: 123i32,
    };

    //let tpl = Template::new(source).unwrap();
    //let output = tpl.render(&person);

    let tpl: Ramhorns = Ramhorns::from_folder_with_extension("./templates", "mustache").unwrap();
    let rendered = tpl.get("hangman.mustache").unwrap().render(&person);
    info!("rendering hangman.mustache with context of {:?}", person);

    Ok(ApiResponse::HTML(rendered))
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

async fn insert_data(State(state): State<MyState>) -> Result<ApiResponse, ApiError> {
    let name = "TestName";
    let number = 123567;

    sqlx::query("INSERT INTO persons (name, number) VALUES ($1, $2)")
        .bind(name)
        .bind(number)
        .execute(&state.pool)
        .await
        .unwrap();

    Ok(ApiResponse::OK)
}

async fn get_data(State(state): State<MyState>) -> Result<ApiResponse, ApiError> {
    let select_query = sqlx::query_as::<_, Person>("SELECT name, number FROM persons");
    let persons: Vec<Person> = select_query.fetch_all(&state.pool).await.unwrap();

    Ok(ApiResponse::JsonData(vec![Message {
        message: format!("Retrieved persons: {:?}", persons),
    }]))
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    //async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let state = MyState { pool };

    sqlx::query(
        r"CREATE TABLE IF NOT EXISTS persons (
            name text,
            number integer
        )",
    )
    .execute(&state.pool)
    .await
    .unwrap();

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/on_state", get(hello_state))
        .route("/insert", get(insert_data))
        .route("/get", get(get_data))
        .with_state(state);

    Ok(router.into())
}
