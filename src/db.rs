// SPDX-FileCopyrightText: 2025 Joost van der Laan <joost@fashionunited.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use refinery::Error as RefineryError;
use std::error::Error as StdError;
use std::str::FromStr;
use tokio_postgres::{Client, Config}; // Renamed to avoid conflict

// Module to embed SQL migration files
mod embedded_migrations {
    use refinery_macros::embed_migrations;
    // This path is relative to the project root (where Cargo.toml is)
    embed_migrations!("./migrations");
}

/// Establishes a connection to the PostgreSQL database using native-tls.
///
/// Reads the connection URL from the "POSTGRES_URL_NON_POOLING" environment variable.
/// Spawns a Tokio task to handle the database connection.
pub async fn connect() -> Result<Client, Box<dyn StdError + Send + Sync + 'static>> {
    let db_url = std::env::var("POSTGRES_URL_NON_POOLING").map_err(|e| {
        let err_msg = format!(
            "Environment variable POSTGRES_URL_NON_POOLING must be set: {}",
            e
        );
        eprintln!("{}", err_msg);
        Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, err_msg))
            as Box<dyn StdError + Send + Sync + 'static>
    })?;

    let pg_config = Config::from_str(&db_url).map_err(|e| {
        let detailed_error_msg = format!(
            "Failed to parse POSTGRES_URL_NON_POOLING. Ensure it is a valid PostgreSQL connection string. Value: '{}'. Error: {}",
            &db_url, e
        );
        eprintln!("{}", detailed_error_msg);
        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, detailed_error_msg)) as Box<dyn StdError + Send + Sync + 'static>
    })?;

    // Set up TLS with native-tls
    let tls_builder = TlsConnector::builder();
    let connector = tls_builder.build().map_err(|e| {
        let detailed_error_msg = format!("Failed to build TLS connector: {}", e);
        eprintln!("{}", detailed_error_msg);
        Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            detailed_error_msg,
        )) as Box<dyn StdError + Send + Sync + 'static>
    })?;
    let tls_connector = MakeTlsConnector::new(connector);

    // Connect using the config and TLS connector
    let (client, connection) = pg_config.connect(tls_connector).await.map_err(|e| {
        let detailed_error_msg = format!(
            "Failed to connect to database with URL: '{}'. Error: {}",
            &db_url, e
        );
        eprintln!("{}", detailed_error_msg);
        Box::new(e) as Box<dyn StdError + Send + Sync + 'static>
    })?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection handler error: {}", e);
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
    embedded_migrations::migrations::runner()
        .run_async(client)
        .await?;
    println!("Database migrations applied successfully.");
    Ok(())
}

// Optional: A combined function for initialization convenience.
/*
pub async fn initialize_database() -> Result<Client, Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Initializing database...");
    let mut client = connect().await?; // Errors will be boxed dyn Error
    run_migrations(&mut client).await?; // This error would be RefineryError, also needs boxing or custom error
    println!("Database initialized successfully.");
    Ok(client)
}
*/
