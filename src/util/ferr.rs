use std::io::Cursor;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::{ ContentType, Status };

pub struct Ferr {
    pub err_type: Status,
    pub err_msg: String
}

impl<'r> Responder<'r, 'static> for Ferr {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        Response::build()
            .sized_body(self.err_msg.len(), Cursor::new(self.err_msg))
            .status(self.err_type)
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

pub fn json_err(msg: String, code: String) -> String {
    return format!("{{ \"code\" : \"{}\", \"message\" : \"{}\" }}", code, msg);
}

pub fn q_err(code: u16, msg: &str) -> Ferr {
    Ferr { err_type: Status::new(code), err_msg: msg.to_string() }
}