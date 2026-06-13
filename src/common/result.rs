use crate::common::{logger, Response};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::Local;
use serde::{Deserialize, Serialize};

/// 成功Code
pub const SUCCESS_CODE: i32 = 200;
/// 错误Code
pub const ERROR_CODE: i32 = 500;
/// 登录错误Code
pub const TOKEN_ERROR_CODE: i32 = 999;

/// 统一返回模型
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultData<T: Serialize> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub timestamp: i64,
}

/// 空返回
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Void;

impl<T: Serialize> ResultData<T> {
    pub fn error(msg: &str) -> Self {
        Self {
            code: ERROR_CODE,
            message: String::from(msg),
            data: None,
            timestamp: Local::now().timestamp_millis(),
        }
    }

    pub fn token_err() -> Self {
        Self {
            code: TOKEN_ERROR_CODE,
            message: String::from("Token无效"),
            data: None,
            timestamp: Local::now().timestamp_millis(),
        }
    }

    pub fn success(data: Option<T>) -> Response<T> {
        Ok(ResultData {
            code: SUCCESS_CODE,
            message: String::from("success"),
            data,
            timestamp: Local::now().timestamp_millis(),
        })
    }

    pub fn success_none() -> Response<T> {
        Ok(ResultData {
            code: SUCCESS_CODE,
            message: String::from("success"),
            data: None,
            timestamp: Local::now().timestamp_millis(),
        })
    }

    pub fn success_data(data: T) -> Response<T> {
        Ok(ResultData {
            code: SUCCESS_CODE,
            message: String::from("success"),
            data: Some(data),
            timestamp: Local::now().timestamp_millis(),
        })
    }
}

impl<T: Serialize> Responder for ResultData<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        match serde_json::to_string(&self) {
            Ok(body) => {
                logger::debug!("{}", body);
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(body)
            },
            Err(err) => {
                logger::error!("Serialization error: {}", err);
                HttpResponse::InternalServerError()
                    .content_type(ContentType::json())
                    .json(serde_json::json!({ "error": "Internal Server Error" }))
            }
        }
    }
}
