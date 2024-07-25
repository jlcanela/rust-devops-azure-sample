use crate::AppState;
use actix_web::{
    error, get, http::{header::ContentType, StatusCode}, post, web::{self, Data, Json}, HttpResponse, Responder
};

use crate::services::*;

use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use derive_more::From;

#[derive(Deserialize)]
struct CreateProjectBody {
    name: String, 
    description: String,
}

#[derive(Serialize, FromRow)]
struct Project {
    id: i32,
    name: String, 
    description: String,
}

pub type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug, From)]
pub enum Error {
    Unknown,
    AuthFailed,
    #[from]
    Io(std::io::Error),
    #[from]
    Parse(std::num::ParseIntError),
    #[from]
    Sqlx(sqlx::Error),
    #[from]
    Serde(serde_json::Error)
}

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::Parse(_) => StatusCode::BAD_REQUEST,
            Error::AuthFailed => StatusCode::FORBIDDEN,
            Error::Unknown | Error::Io(_) | Error::Sqlx(_) | Error::Serde(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[post("/api/projects")]
async fn create_project(state: Data<AppState>, body: Json<CreateProjectBody>) -> impl Responder {
    let project = body.into_inner();

    match sqlx::query_as::<_, Project>(
        "INSERT INTO projects (name, description, owned_by, created_by, updated_by)
        VALUES ($1, $2, 2, 2, 2)
        RETURNING id, name, description"
    )
    .bind(project.name)
    .bind(project.description)
    .fetch_one(&state.db)
    .await {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(error) => HttpResponse::InternalServerError().json(format!("Error: {:?}", error))
    }
}

#[get("/api/projects")]
async fn list_projects(state: Data<AppState>) -> Result<String> {
    let _token = state.current_token();

    let projects = sqlx::query_as::<_, Project>(
                "SELECT id, name, description from projects"
    )
    .fetch_all(&state.db)
    .await?;

    let json =  serde_json::to_string(&projects)?;
    Ok(json)
}

#[get("/api/projects/{id}")]
async fn get_project(state: Data<AppState>, path: web::Path<String>) -> Result<String> {
    println!("get_project");
    let project_id: String = path.into_inner();
    let id = project_id.parse::<i64>()?;
    let project = sqlx::query_as::<_, Project>(
        "SELECT id, name, description from projects
        WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;
    
    let token = state.current_token();
    if !state.permission.is_authorized(token, Action::ViewProject, &project) {
        return Err(Error::AuthFailed)
    }

    let json =  serde_json::to_string(&project)?;
    Ok(json)
}

#[get("/status")]
async fn status(_state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("OK")
}
