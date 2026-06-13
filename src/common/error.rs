use crate::common::logger;
use crate::common::result::{ResultData, Void};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::{DbErr, TransactionError};
use std::error::Error;
use std::fmt;

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

/// Server Error
#[derive(Debug)]
pub enum ServerError {
    InnerError(BoxError),
    BusinessError(&'static str),
    BusinessStrError(String),
    TokenError,
    OptimisticLock,
    HttpStatus(u16, String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InnerError(err) => write!(f, "发生内部错误: {}", err),
            Self::BusinessError(msg) => write!(f, "业务出错: {msg}"),
            Self::BusinessStrError(msg) => write!(f, "业务出错: {msg}"),
            Self::OptimisticLock => write!(f, "数据已被更改"),
            Self::TokenError => write!(f, "登录Token失效"),
            Self::HttpStatus(status, msg) => write!(f, "返回Http状态码 : {status}, 信息: {msg}"),
        }
    }
}

impl Error for ServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::BusinessError(_) => None,
            Self::BusinessStrError(_) => None,
            Self::InnerError(e) => Some(e.as_ref()),
            Self::TokenError => None,
            Self::OptimisticLock => None,
            Self::HttpStatus(_, _) => None,
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InnerError(err) => {
                logger::error!("发生内部错误: {}", err);
                HttpResponse::Ok().json(ResultData::<Void>::error("Internal Server Error"))
            }
            Self::BusinessError(msg) => {
                HttpResponse::Ok().json(ResultData::<Void>::error(msg))
            }
            Self::BusinessStrError(msg) => {
                HttpResponse::Ok().json(ResultData::<Void>::error(msg))
            }
            Self::TokenError => {
                HttpResponse::Ok().json(ResultData::<Void>::token_err())
            }
            ServerError::OptimisticLock => {
                HttpResponse::Ok().json(ResultData::<Void>::error("数据已被更改, 请重新提交"))
            }
            Self::HttpStatus(status, msg) => {
                HttpResponse::build(StatusCode::from_u16(*status).unwrap()).content_type(ContentType::plaintext()).body(msg.to_string())
            }
        }
    }
}

impl From<DbErr> for ServerError {
    fn from(err: DbErr) -> Self {
        ServerError::InnerError(Box::new(err))
    }
}

impl From<TransactionError<ServerError>> for ServerError {
    fn from(err: TransactionError<ServerError>) -> Self {
        match err {
            TransactionError::Connection(e) => ServerError::InnerError(Box::new(e)),
            TransactionError::Transaction(e) => e,
        }
    }
}

impl From<serde_json::error::Error> for ServerError {
    fn from(value: serde_json::Error) -> Self {
        ServerError::InnerError(Box::new(value))
    }
}

impl From<reqwest::Error> for ServerError {
    fn from(value: reqwest::Error) -> Self {
        ServerError::InnerError(Box::new(value))
    }
}
