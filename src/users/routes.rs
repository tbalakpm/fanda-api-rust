use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::users::{User, UserRequest};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all).service(find_by_id).service(create);
    // .service(update)
    // .service(delete);
}

#[get("/users")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::find_all(db_pool.get_ref()).await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            error!("error fetching users: {}", err);
            HttpResponse::InternalServerError().body("Error reading users from database")
        }
    }
}

#[get("/users/{id}")]
async fn find_by_id(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            error!("error fetching user by id: {}", err);
            HttpResponse::InternalServerError().body("Error reading user by id from database")
        }
    }
}

#[post("/users")]
async fn create(user: web::Json<UserRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::create(user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            error!("error creating role: {}", err);
            HttpResponse::InternalServerError().body("Error creating new role")
        }
    }
}

// #[put("/roles/{id}")]
// async fn update(
//     id: web::Path<i32>,
//     role: web::Json<RoleRequest>,
//     db_pool: web::Data<PgPool>,
// ) -> impl Responder {
//     let result = Role::update(*id, role.into_inner(), db_pool.get_ref()).await;
//     match result {
//         Ok(role) => HttpResponse::Ok().json(role),
//         Err(err) => {
//             error!("error updating role: {}", err);
//             HttpResponse::InternalServerError().body("Error updating role")
//         }
//     }
// }

// #[delete("/roles/{id}")]
// async fn delete(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
//     let result = Role::delete(*id, db_pool.get_ref()).await;
//     match result {
//         Ok(rows_deleted) => {
//             if rows_deleted > 0 {
//                 let msg = format!("Successfully deleted {} record(s)", rows_deleted);
//                 HttpResponse::Ok().body(msg)
//             } else {
//                 HttpResponse::NotFound().body("Role not found")
//             }
//         }
//         Err(err) => {
//             error!("error creating role: {}", err);
//             HttpResponse::InternalServerError().body("Error deleting role")
//         }
//     }
// }
