use rocket_contrib::json;
use rocket::{
    request::Request,
    response::{ self, Response, Responder },
    http::{ Status, ContentType },
};
use self::ApiError::*;
use serde::Serialize;

pub type ApiResponse<T> = Result<ApiSuccess<T>, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    ApiError,
}

#[derive(Serialize)]
pub struct ApiSuccess<T: Serialize>(pub T);

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
            "error": self.error_message(),
        }).respond_to(&req).unwrap();

        Response::build_from(json_response)
            .status(self.status())
            .header(ContentType::JSON)
            .ok()
    }
}

impl ApiError {
    fn status(&self) -> Status {
        match self {
            ApiError => Status::InternalServerError,
        }
    }

    fn error_message(&self) -> &str {
        match self {
            ApiError => "Interal Error",
        }
    }
}
