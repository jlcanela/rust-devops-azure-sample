use crate::AppState;
use actix_web::{
    get, post,
    web,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::services::*;

use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow, Pool, Postgres};

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

#[post("/api/projects")]
async fn create_project(state: Data<AppState>, body: Json<CreateProjectBody>) -> impl Responder {
    dbg!(&state.permission);
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
async fn list_projects(state: Data<AppState>) -> impl Responder {
    let _token = state.current_token();

    match sqlx::query_as::<_, Project>(
                "SELECT id, name, description from projects"
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error))
    }
}

#[get("/api/project/{id}")]
async fn get_project(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    let project_id: String = path.into_inner();

    async fn fetch(db: &Pool<Postgres>, permission: &Permission, token: Option<String>, id: String) -> Result<Project, String> {
        let id = id.parse::<i64>().map_err(|err| err.to_string())?;
        let project = sqlx::query_as::<_, Project>(
                "SELECT id, name, description from projects
                WHERE id = $1"
            )
            .bind(id)
            .fetch_one(db)
            .await
            .map_err(|err| err.to_string())?;

       
        let permission = permission.is_authorized(token, Action::ViewProject, &project);
        if permission {
            Ok(project)
        } else {
            Err("Authorization Failed".to_string())
        }
    
    }

    let token = state.current_token();
    match fetch(&state.db, &state.permission, token, project_id).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) if err == "Authorization Failed" => HttpResponse::Forbidden().json("Forbidden"),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error))
    }
}

#[get("/status")]
async fn status(_state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("OK")
}
