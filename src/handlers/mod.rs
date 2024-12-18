use crate::models::CreateUserRequest;
use crate::services;
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};

pub async fn create_user(
    db_pool: web::Data<sqlx::PgPool>,
    user_data: web::Json<CreateUserRequest>,
) -> impl Responder {
    info!("📝 Creating user with name: {}", user_data.name);
    match services::create_user(db_pool.get_ref(), user_data.into_inner()).await {
        Ok(user) => {
            info!("✅ User created successfully: {}", user.name);
            HttpResponse::Ok().json(user)
        }
        Err(services::ServiceError::DatabaseError(msg)) => {
            error!("❌ Database error while creating user: {}", msg);
            HttpResponse::InternalServerError().body(msg)
        }
        Err(services::ServiceError::ValidationError(msg)) => {
            error!("⚠️ Validation error while creating user: {}", msg);
            HttpResponse::BadRequest().body(msg)
        }
    }
}

pub async fn get_users(db_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    info!("📥 Fetching all users...");
    match services::get_all_users(db_pool.get_ref()).await {
        Ok(users) => {
            info!("✅ Successfully retrieved {} users", users.len());
            HttpResponse::Ok().json(users)
        }
        Err(err) => {
            error!("❌ Failed to fetch users: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Error: {:?}", err))
        }
    }
}
