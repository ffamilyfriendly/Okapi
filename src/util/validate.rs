use rocket::http::Status;
use serde_json::json;
use crate::util;

pub fn verify_respond(e: validator::ValidationErrors) -> util::ferr::Ferr {
    util::ferr::Ferr { err_msg: json!(e).to_string(), err_type: Status::BadRequest }
}
