use axum::{http::StatusCode,
    response::{IntoResponse,Response},
    Json,
}
use thiserror::Error;
use serde_json::json;

#[derive(Error,Debug)]
pub enum AppError{
    #[error("Database Error:{0}")]
    Database(#[from] sqlx::Error)

    #[error("Fairscale Error:{0}")] 
    FairScale(string)

    #[error("Invalid request:{0}")]
    BadRequest(string)

    #[error("Not found")]
    NotFound,

    #[error("Internal Server Error")]
    Internal,
}

impl IntoResponse for AppError{
    fn into_response(self)->Response{
        let (status,message)=match self{
            AppError::Database(_)=>(StatusCode::INTERNAL_SERVER_ERROR,"Database error"),
            AppError::FairScale(msg)=>(StatusCode::BAD_GATEWAY,msg),
            AppError::BadRequest(msg)=>(StatusCode::BAD_GATEWAY,msg),
            AppError::NotFound=>(StatusCode::NOT_FOUND,"Not found error"),
            AppError::Internal=>(StatusCode::INTERNAL_SERVER_ERROR,"Internal Error")
        }

        let body = Json(json!({"error":message}));
        (status,body).into_response()
    }
}