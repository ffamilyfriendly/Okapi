use rusqlite::{Connection};
use serde::{Deserialize, Serialize};
use std::time::{ SystemTime, UNIX_EPOCH };
use crate::util::gen_id::{gen_id};

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
pub struct Invite {
    pub id: String,
    pub created_by: u16,
    pub user_flag: u16,
    pub expires: u64,
    pub uses: i16
}

impl Invite {
    pub fn Use(&mut self) {
        self.uses -= 1;

        if self.uses <= 0 {
            delete_invite(&self.id);
        } else {
            update_invite(self);
        }
    }
}

pub fn delete_invite(id: &String) -> Result<bool, rusqlite::Error> {
    let connection = match Connection::open("data.sqlite") {
        Ok(con) => con,
        Err(e) => return Err(e)
    };

    match connection.execute("DELETE FROM invites WHERE id = ?", [id]) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

pub fn update_invite(i: &Invite) -> Result<bool, rusqlite::Error> {
    let connection = match Connection::open("data.sqlite") {
        Ok(con) => con,
        Err(e) => return Err(e)
    };

    match connection.execute("UPDATE invites SET user_flag = ?, expires = ?, uses = ? WHERE id = ?", [ i.user_flag.to_string(), i.expires.to_string(), i.uses.to_string(), i.id.to_string() ]) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

fn get_invite_struct(row: &rusqlite::Row<'_>) -> Result<Invite, rusqlite::Error> {
    Ok ( Invite {
        id: row.get(0)?,
        created_by: row.get(1)?,
        user_flag: row.get(2)?,
        expires: row.get(3)?,
        uses: row.get(4)?
    } )
}

pub fn get_invite(id: &String) -> Option<Invite> {
    let connection = match Connection::open("data.sqlite") {
        Ok(con) => con,
        Err(_) => return None
    };

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

    match connection.query_row("SELECT * FROM invites WHERE id = ?", [&id], |r| get_invite_struct(r)) {
        Ok(r) => {

            if r.expires > 0 && r.expires < now as u64 {
                return None;
            }
            
            return Some(r);
        },
        Err(_) => None
    }
}

pub fn generate_invite(created_by: u16, user_flag: u16, expires: u64, uses: i16) -> Result<String, bool> {
    
    let connection = match Connection::open("data.sqlite") {
        Ok(con) => con,
        Err(_) => { return Err(false) }
    };

    let id: String = gen_id();

    match connection.execute("INSERT INTO invites VALUES (?, ?, ?, ?, ?)", [&id, &created_by.to_string(), &user_flag.to_string(), &expires.to_string(), &uses.to_string()]) {
        Ok(_) => Ok(id),
        Err(_) => Err(false)
    }
}

// this is copied from the users thing
pub fn get_invites(creator: Option<u16>) -> Result<Vec<Invite>, rusqlite::Error> {
    let con = match Connection::open("data.sqlite") {
        Ok(con) => con,
        Err(e) => { return Err(e) }
    };

    let mut sql: String = "SELECT * FROM invites".to_string();
    if creator.is_some() { sql += &format!(" WHERE created_by = {}", creator.unwrap_or(0)).to_string(); };

    let mut statement = match con.prepare(sql.as_str()) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    
    let user_iter = match statement.query_map([], |row| get_invite_struct(row)) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };

    let mut users: Vec<Invite> = Vec::new();

    for user in user_iter {
        match user {
            Ok(s) => {
                users.push(s)
            },
            Err(e) => return Err(e)
        };
    };

    Ok(users)
}