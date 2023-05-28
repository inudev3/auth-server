use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::error::ParseError;
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use uuid::Error;

#[derive(Debug, Display)]
pub enum ServiceError{
    #[display(fmt="Internal Server Error")]
    InternalServerError,
    #[display(fmt= "BadRequest: {}", _0)]
    BadRequest(String),
    #[display(fmt= "Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServiceError{
    fn error_response(&self) -> HttpResponse<BoxBody> {
        use ServiceError::*;
        match self{
            InternalServerError=>{
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            },
            BadRequest(ref message)=>HttpResponse::BadRequest().json(message),
            Unauthorized => HttpResponse::Unauthorized().json("Unauthorized")
        }
    }
}
impl From<ParseError> for ServiceError{
    fn from(_:ParseError)->ServiceError{
        ServiceError::BadRequest("Invalid UUID".to_string())
    }
}
impl From<r2d2::Error> for ServiceError{
    fn from(_:r2d2::Error)->ServiceError{
        ServiceError::InternalServerError
    }
}
impl From<DBError> for ServiceError{
    fn from(error:DBError)->ServiceError{
        match error{
            DBError::DatabaseError(kind, info)=>{
                if let DatabaseErrorKind::UniqueViolation = kind{
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message)
                }
                ServiceError::InternalServerError
            }
            _=>ServiceError::InternalServerError
        }
    }
}
impl From<uuid::Error> for ServiceError{
    fn from(err: Error) -> Self {
        ServiceError::InternalServerError
    }
}