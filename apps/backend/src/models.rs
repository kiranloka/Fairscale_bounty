use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub wallet_address: String,
    pub fair_score: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub sub_score: Option<serde_json::Value>,
    pub score_fetched_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FairScoreResponse {
    pub fair_score: i32,
    #[serde(rename = "sub_scores")]
    pub sub_scores: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Bounty {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub reward_amount: Decimal,
    pub reward_mint: Option<String>,
    pub min_fair_score: i32,
    pub deadline: Option<DateTime<Utc>>,
    pub status: String,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBounty {
    pub title: String,
    pub description: String,
    pub reward_amount: Decimal,
    pub reward_mint: Option<String>,
    pub min_fair_score: i32,
    pub deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Application {
    pub id: Uuid,
    pub bounty_id: Uuid,
    pub applicant_id: Uuid,
    pub status: String, //Status of approval
    pub application_fair_score: i32,
    pub created_at: DateTime<Utc>,
}
