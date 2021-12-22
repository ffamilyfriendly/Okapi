use serde::{ Serialize, Deserialize };
use jsonwebtoken::{ encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey };
use std::time::{ SystemTime, UNIX_EPOCH };

/* User JWT struct */
#[ derive(Serialize, Deserialize) ]
pub struct Claims {
    pub exp: u128, /* when token expires */
    pub sub: String, /* subject (email) */
    /* okapi specific properties */
    pub uid: u16, /* User id */
    pub permissions: u16 /* u16 int leaves us with 16 diff perms / flags. Should be enough */
}

impl std::convert::From<&str> for Claims {
    fn from(t: &str) -> Self {
        decode::<Claims>(&t, &DecodingKey::from_secret(include_bytes!("secret.txt")), &Validation::default()).unwrap().claims
    }
}

// I really could not find a good variable name. Should fix this but I doubt anyone besides me is reading this comment anyhow
pub struct CoolStructThing {
    pub claim: Claims,
    pub token: String
}

pub fn get_cool_struct_thing(row: &rusqlite::Row<'_>) -> Result<CoolStructThing, rusqlite::Error> {
    let uid: u16 = row.get(0)?;
    let flag: u16 = row.get(1).unwrap_or(0);
    let email: String = row.get(3)?;
    let pswd: String = row.get(4)?;
    //                                                                            #    about half a year     #
    //                                                                            vvvvvvvvvvvvvvvvvvvvvvvvvvvv
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + 1000 * 60 * 60 * 24 * 30 * 6;
    let claim: Claims = Claims { exp: exp, sub: email, uid: uid, permissions: flag };
    Ok(CoolStructThing { claim: claim, token: pswd })
}

pub fn gen_token(user: &Claims) -> String {
    let token = encode(&Header::new(Algorithm::default()), user, &EncodingKey::from_secret(include_bytes!("secret.txt")));
    token.unwrap()
}

pub fn passes(token: String) -> bool {
    match decode::<Claims>(&token, &DecodingKey::from_secret(include_bytes!("secret.txt")), &Validation::default()) {
        Ok(_) => true,
        Err(_) => false
    }
}

use rocket::request::{ Outcome, Request, FromRequest };
use rocket::http::Status;

pub struct Token(pub Claims);

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request( req: &'r Request<'_> ) -> Outcome<Self, Self::Error> {
        let token = req.headers().get_one("token");
        match token {
            Some(token) => {
                if !passes(token.to_string()) { return Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid)); }
                Outcome::Success(Token(token.into()))
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing))
        }
    }
}