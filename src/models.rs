use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl User {
    // Hash the password before storing it
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    // Verify the password
    pub fn verify_password(stored_hash: &str, password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, stored_hash)
    }
}
