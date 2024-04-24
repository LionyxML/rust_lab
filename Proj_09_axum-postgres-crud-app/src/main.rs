use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use sqlx::{postgres::PgPoolOptions, PgPool};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // expose the environment variables

    dotenvy::dotenv().expect("Unable to access .env file");

    // set variables from the environment variables

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the env file");

    // create the database pool

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Can't connect to the database");

    // create our TCP listener

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Can't init the server");

    println!("Listening on {}", listener.local_addr().unwrap());

    // compose the routes

    let app = Router::new().route("/", get(|| async { "Hello World" }));

    // serve the application

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

