use rocket::http::{ Header, Method, Status };
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
       }
    }


    async fn on_response<'r>(&self, req: &'r Request<'_>, response: &mut Response<'r>) {

        // only cors makes OPTION calls therefore I can do this. I am a genious. My brain is large. I hav big iq
        if req.method() == Method::Options {
            response.set_status(Status::Accepted);
        }

        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}