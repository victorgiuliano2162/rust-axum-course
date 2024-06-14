use crate::{
    ctx::{self, Ctx},
    error::ClientError,
    Error, Result,
};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

pub async fn log_resquest(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    
}

//Option::none does not get serialized although Option::Some(T) does it
#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      //uuid string formatted
    timestamp: String, //should be iso8601

    //user and context attributes
    user_id: Option<u64>,

    //http request attributes
    req_path: String,
    req_method: String,

    //Error atributes
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
