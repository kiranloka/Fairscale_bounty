use axum::{
    extract::State,
    routing::get,
    Json,
    http::HttpHeaderMap,
}

use crate::{state::AppState, models::{FairScore, FairScoreResponse},errors::AppError};
use reqwest::header;


pub fn get_fa
