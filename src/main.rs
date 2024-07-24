use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod schema;
mod models;

use models::{NewUser, User};

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect("Error connecting to the database")
}

fn create_user(conn: &SqliteConnection, username: &str, password: &str) -> Result<User, diesel::result::Error> {
    let hashed_password = User::hash_password(password).expect("Failed to hash password");

    let new_user = NewUser {
        username,
        password: &hashed_password,
    };

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(conn)?;

    // Query the user back to return it
    schema::users::table
        .filter(schema::users::username.eq(username))
        .first(conn)
}

fn verify_user(conn: &SqliteConnection, username: &str, password: &str) -> Result<bool, diesel::result::Error> {
    let user: User = schema::users::table
        .filter(schema::users::username.eq(username))
        .first(conn)?;

    User::verify_password(&user.password, password).map_err(|e| diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UnableToSendCommand, Box::new(e)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(auth_middleware)
            // Add routes and other middleware here
            .service(
                web::resource("/protected")
                    .route(web::get().to(|| async { "Protected route" }))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}