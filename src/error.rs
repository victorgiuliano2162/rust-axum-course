use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

}

// -- Error boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
} 

impl std::error::Error for Error {}

// -- Error boilerplate

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("-> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
    //Never pass your server error to your client
}
