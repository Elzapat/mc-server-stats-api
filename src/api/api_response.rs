use rocket_contrib::json;
use rocket::{
    request::Request,
    response::{ self, Response, Responder },
    http::{ Status, ContentType },
};
use std::fmt;
use self::ApiError::*;
use serde::Serialize;

pub type ApiResponse<T> = Result<ApiSuccess<T>, ApiError>;

pub enum ApiError {
    ApiError,
}

#[derive(Serialize)]
struct ApiSuccess<T: Serialize>(T);

impl<'r, T: Serialize> Responder<'r> for ApiSuccess<T> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let json_response = json!({
            "success": true,
            "status": Status::Ok.to_string(),
            "data": self,
        }).respond_to(&req).unwrap();

        Response::build_from(json_response)
            .status(Status::Ok)
            .header(ContentType::JSON)
            .ok()
    }
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let json_response = json!({
            "success": false,
            "status": self.status().to_string(),
            "error": self.to_string(),
        }).respond_to(&req).unwrap();

        Response::build_from(json_response)
            .status(self.status())
            .header(ContentType::JSON)
            .ok()
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ApiError => "Internal Error",
        })
    }
}

impl ApiError {
    fn status(&self) -> Status {
        match self {
            ApiError => Status::InternalServerError,
        }
    }
}
