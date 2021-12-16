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
    pub permissions: u8 /* u8 int leaves us with 255 diff perms / flags. Should be plenty */
}

pub struct CoolStructThing {
    pub claim: Claims,
    pub token: String
}

/*
CREATE TABLE IF NOT EXISTS users (
                    id	INTEGER NOT NULL, /* id of user, going to be the relative tie to anything user related */
                    flag INTEGER, /* flag of int type. Going to specify if user is entitled to perms/functions like admin and whatevs */
                    username	TEXT NOT NULL, /* the name that will displayed, cannot be logged in with */
                    email       TEXT NOT NULL UNIQUE, /* the users email, will be used to log in. */
                    password	TEXT NOT NULL, /* password crypt. password will NEVER be stored plain */
                    PRIMARY KEY(id AUTOINCREMENT)
                );

*/

pub fn get_cool_struct_thing(row: &rusqlite::Row<'_>) -> Result<CoolStructThing, rusqlite::Error> {
    let uid: u16 = row.get(0)?;
    let flag: u8 = row.get(1).unwrap_or(0);
    let email: String = row.get(3)?;
    let pswd: String = row.get(4)?;
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let claim: Claims = Claims { exp: exp, sub: email, uid: uid, permissions: flag };
    Ok(CoolStructThing { claim: claim, token: pswd })
}

pub fn genToken(user: &Claims) -> String {
    let token = encode(&Header::new(Algorithm::default()), user, &EncodingKey::from_secret(include_bytes!("secret.txt")));
    token.unwrap()
}