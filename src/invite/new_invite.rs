use crate::user::userutil::{ Token };
use crate::util::{ permissions, ferr };
use rocket::response::status::Created;
use serde::{Deserialize, Serialize};
use rusqlite::{Connection};
use rocket::serde::json::Json;
use rocket::State;
use rand_core::{ OsRng, RngCore };

/*
                CREATE TABLE IF NOT EXISTS invites (
                    id	TEXT NOT NULL UNIQUE,
                    created_by	INTEGER NOT NULL, /* id of the user who created the invite */
                    user_flag INTEGER, /* gives user these perms after signup */
                    expires	INTEGER, /* unix timestamp when invite expires. If invite does not expire with time, set to -1 */
                    uses	INTEGER, /* how many times the invite can be used. If it can be used unlimited times, set to -1 */
                    PRIMARY KEY(id)
                );
*/

#[derive(Serialize, Deserialize)]
pub struct NewInvite {
    user_flag: Option<u16>,
    expires: Option<i128>,
    uses: Option<i16>
}

fn gen_id() -> String {
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    let random_u64 = OsRng.next_u64();
    random_u64.to_string()
}

#[post("/", data="<input>")]
pub fn new_invite(state: &State<crate::config>, user: Token, input: Json<NewInvite>) -> Result<Created<String>, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::GenerateInvite) {
        return Err(ferr::q_err(403, "endpoint requires GenerateInvite permission"))
    }

    let connection = match Connection::open("data.sqlite") {
        Ok(con) => con,
        Err(_) => return Err(ferr::q_err(500, "something went wrong"))
    };
    let user_flag: u16 = input.user_flag.unwrap_or(0);
    let expires: i128 = input.expires.unwrap_or(-1);
    let uses: i16 = input.uses.unwrap_or(-1);
    let id: String = gen_id();
    let created_by = user.0.uid;

    match connection.execute("INSERT INTO invites VALUES (?, ?, ?, ?, ?)", [&id, &created_by.to_string(), &user_flag.to_string(), &expires.to_string(), &uses.to_string()]) {
        Ok(_) => Ok(Created::new(format!("{}/invite/{}", &state.inner().hostname, &id))),
        Err(_) => Err(ferr::q_err(500,"something went wrong"))
    }
}