use actix_web::{post, web, HttpResponse, Responder};
use sqlx::MySqlPool;

// Assuming models.rs and utils.rs are in the same directory
use crate::models;
use crate::utils;

use models::{User, StoredUser};
use utils::{hash_password, verify_password, generate_token};

#[post("/register")]
pub async fn register(user: web::Json<User>, pool: web::Data<MySqlPool>) -> impl Responder {
    let hashed_password = hash_password(&user.password);
    
    let result = sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(&user.username)
        .bind(&hashed_password)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User registered successfully"
        })),
        Err(e) => {
            if e.to_string().contains("Duplicate entry") {
                HttpResponse::Conflict().json(serde_json::json!({
                    "error": "Username already exists"
                }))
            } else {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to register user"
                }))
            }
        }
    }
}

#[post("/login")]
pub async fn login(user: web::Json<User>, pool: web::Data<MySqlPool>) -> impl Responder {
    let user_result = sqlx::query_as::<_, StoredUser>("SELECT id, username, password FROM users WHERE username = ?")
        .bind(&user.username)
        .fetch_optional(pool.get_ref())
        .await;

    match user_result {
        Ok(Some(stored_user)) => {
            if verify_password(&user.password, &stored_user.password) {
                let token = generate_token(&stored_user.username);
                HttpResponse::Ok().json(serde_json::json!({
                    "token": token
                }))
            } else {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid credentials"
                }))
            }
        },
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "An error occurred while processing your request"
        })),
    }
}