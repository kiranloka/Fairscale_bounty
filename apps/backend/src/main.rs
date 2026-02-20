use axum::{
    routing:get,
    Router,Server,
}

use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{fmt,prelude::EnvFilter};

mod router;
mod db;
mod models;
mod states;
mod errors;


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
    tracing_subscriber::fmt().with_env_filter(Envfilter::from_default_env()).init();


    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pool = PgPool::connect(&database_url).await expect("failed to connect to the database");

    sqlx::migrate!("./migrations").run(&pool).await?

    let state = AppState{
        db:pool,
        http_client: request::Client::new(),
    };

    let app = Router::new()
        .route("/",get (routes))
}

