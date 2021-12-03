use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
// use sqlx::SqlitePool;
use sqlx::PgPool;

use crate::roles::{Role, RoleRequest};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all)
        .service(find_by_id)
        .service(create)
        .service(update)
        .service(delete);
}

#[get("/roles")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Role::find_all(db_pool.get_ref()).await;
    match result {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(err) => {
            error!("error fetching roles: {}", err);
            HttpResponse::InternalServerError().body("Error reading roles from database")
        }
    }
}

#[get("/roles/{id}")]
async fn find_by_id(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Role::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(Some(role)) => HttpResponse::Ok().json(role),
        Ok(None) => HttpResponse::NotFound().body("Role not found"),
        Err(err) => {
            error!("error fetching role by id: {}", err);
            HttpResponse::InternalServerError().body("Error reading role by id from database")
        }
    }
}

#[post("/roles")]
async fn create(role: web::Json<RoleRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Role::create(role.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(err) => {
            error!("error creating role: {}", err);
            HttpResponse::InternalServerError().body("Error creating new role")
        }
    }
}

#[put("/roles/{id}")]
async fn update(
    id: web::Path<i32>,
    role: web::Json<RoleRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Role::update(*id, role.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(err) => {
            error!("error updating role: {}", err);
            HttpResponse::InternalServerError().body("Error updating role")
        }
    }
}

#[delete("/roles/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Role::delete(*id, db_pool.get_ref()).await;
    match result {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                let msg = format!("Successfully deleted {} record(s)", rows_deleted);
                HttpResponse::Ok().body(msg)
            } else {
                HttpResponse::NotFound().body("Role not found")
            }
        }
        Err(err) => {
            error!("error creating role: {}", err);
            HttpResponse::InternalServerError().body("Error deleting role")
        }
    }
}
