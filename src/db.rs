// SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use tokio_postgres::{Client, Error as PgError, NoTls};
use refinery::Error as RefineryError;

// Module to embed SQL migration files
mod embedded_migrations {
    use refinery_macros::embed_migrations;
    // This path is relative to the project root (where Cargo.toml is)
    embed_migrations!("./migrations");
}

/// Establishes a connection to the PostgreSQL database.
///
/// Reads the connection URL from the "POSTGRES_URL_NON_POOLING" environment variable.
/// Spawns a Tokio task to handle the database connection.
pub async fn connect() -> Result<Client, PgError> {
    let db_url = std::env::var("POSTGRES_URL_NON_POOLING")
        .expect("Environment variable POSTGRES_URL_NON_POOLING must be set");

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

/// Runs database migrations using the embedded migration files.
///
/// # Arguments
/// * `client` - A mutable reference to a `tokio_postgres::Client`.
pub async fn run_migrations(client: &mut Client) -> Result<(), RefineryError> {
    println!("Applying database migrations...");
    embedded_migrations::migrations::runner().run_async(client).await?;
    println!("Database migrations applied successfully.");
    Ok(())
}

// Optional: A combined function for initialization convenience.
// This function returns a more generic error type to encompass both PgError and RefineryError.
// You might want to define a custom error type for your application instead.
/*
pub async fn initialize_database() -> Result<Client, Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Initializing database...");
    let mut client = connect().await.map_err(Box::new)?;
    run_migrations(&mut client).await.map_err(Box::new)?;
    println!("Database initialized successfully.");
    Ok(client)
}
*/