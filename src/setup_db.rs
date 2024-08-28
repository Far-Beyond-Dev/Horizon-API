use sqlx::sqlite::SqlitePool;
use std::env;
use std::fs::File;
use std::path::Path;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub async fn setup_db() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:mydb.db".to_string());
    let db_path = Path::new("mydb.db");
    
    let is_new_db = !db_path.exists();
    
    if is_new_db {
        // Attempt to create the file if it doesn't exist
        if let Err(e) = File::create(db_path) {
            eprintln!("Error creating database file: {}", e);
            panic!("Unable to create database file");
        }
    }

    let pool = match SqlitePool::connect(&database_url).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            panic!("Unable to establish a database connection");
        }
    };

    // Create the 'servers' table if it does not exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS servers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            status TEXT NOT NULL,
            players INTEGER NOT NULL,
            cpu REAL NOT NULL,
            memory REAL NOT NULL
        )"
    ).execute(&pool).await.unwrap();

    // Add test servers only if it's a new database
    if is_new_db {
        add_test_servers(&pool).await;
    }

    pool
}

async fn add_test_servers(pool: &SqlitePool) {
    // Use SmallRng seeded from entropy
    let mut rng = SmallRng::from_entropy();
    for i in 1..=100 {
        let name = format!("Server {}", i);
        let status = if rng.gen_bool(0.8) { "Online" } else { "Offline" };
        let players = rng.gen_range(0..100);
        let cpu = rng.gen_range(0.0..100.0);
        let memory = rng.gen_range(0.0..100.0);
        sqlx::query(
            "INSERT INTO servers (name, status, players, cpu, memory)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&name)
        .bind(&status)
        .bind(players)
        .bind(cpu)
        .bind(memory)
        .execute(pool)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to insert test server {}: {}", i, e);
            panic!("Database insertion failed");
        });
    }
    println!("Added 100 test servers to the database.");
}