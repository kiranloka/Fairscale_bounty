use sqlx:PgPool;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState{
    pub db:PgPool,
    pub http_client:Client,
}