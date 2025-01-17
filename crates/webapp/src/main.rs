use actix_web::{
    dev:: ServiceRequest,
    error:: Error,
    web::{self, Data},
    App, HttpMessage, HttpServer,
};

use std::str::FromStr;
use dotenv::dotenv;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
// use sqlx::Sqlite;
// use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use std::path::{Path};
mod services;
use services::{basic_auth, create_article, create_user};


pub struct AppState {
    db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id: i32,
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req.app_data::<bearer::Config>().cloned().unwrap_or_default().scope("");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

trait FindDatabaseType {
    fn db_type(&self) -> DatabaseType;
}

enum DatabaseType {
    //Sqlite,
    Postgres
}

impl FindDatabaseType for String {
    fn db_type(&self) -> DatabaseType {
        DatabaseType::Postgres
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Configure the connection options
    let db_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set!");

    let db_type = db_url.db_type();

    
    let pool: Pool<_> = match db_type {
        // DatabaseType::Sqlite => {
        //     let options = SqliteConnectOptions::from_str(&db_url)
        //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        //     .create_if_missing(true);
        
        //     // Create the connection pool
        //     SqlitePool::connect_with(options).await
        //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        // } 
    
        DatabaseType::Postgres => {                    
            let options = PgConnectOptions::from_str(&db_url)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            // Create the connection pool
            PgPoolOptions::new()
            .max_connections(5) // Adjust the number of max connections as needed
            .connect_with(options)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        }
    };

    use sqlx::migrate::Migrator;
    let migration_path = Path::new("./sql-migrations").to_path_buf();
    let migrator = Migrator::new(migration_path).await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    migrator.run(&pool).await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(basic_auth)
            .service(create_user)
            .service(
                web::scope("")
                    .wrap(bearer_middleware)
                    .service(create_article)   
            )
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

