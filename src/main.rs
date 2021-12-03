#[macro_use]
extern crate log;

use std::env;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
// use sqlx::sqlite::SqlitePoolOptions;
use sqlx::postgres::PgPoolOptions;

mod roles;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
    Welcome to Fanda API.

    Available routes:
    Roles:
    ------
    GET     /roles      ->  list of all roles
    POST    /roles      ->  create new role, example: {"name": "Administrator", "is_enabled": false}
    GET     /roles/{id} ->  show one role with requested id
    PUT     /roles{id}  ->  update role with requested id, example: {"name": "Administrator", "is_enabled": true}
    DELETE  /roles/{id} ->  delete role with requested id
    "#,
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT")
        .expect("PORT is not set in .env file")
        .parse::<u16>()
        .expect("PORT should be a u16 type");

    info!("Using PostgreSQL database at: {}", &database_url);
    // let db_pool = SqlitePoolOptions::new().connect(&database_url).await?;
    let db_pool = PgPoolOptions::new().connect(&database_url).await?;
    // .expect("Database connection failed");

    // sqlx::query!(r#"SELECT id AS "id: i32" FROM roles LIMIT 1"#)
    //     .fetch_optional(&db_pool)
    //     .await
    //     .expect("No connection to the database");

    let server = HttpServer::new(move || {
        App::new()
            // pass database pool to application so we can access it inside handlers
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .configure(roles::init) // init role routes
    })
    .bind((host, port))?;

    info!("Starting server");
    server.run().await?;

    Ok(())
}
