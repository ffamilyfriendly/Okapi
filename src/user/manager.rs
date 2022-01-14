use rusqlite::{ Connection };
use serde::{ Serialize, Deserialize };
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use std::time::{ SystemTime, UNIX_EPOCH };

pub fn date_now_string() -> String {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string()
}

fn get_conn() -> Result<Connection, rusqlite::Error> {
    Connection::open("data.sqlite")
}

pub fn reg_session(token: String, user: u16) -> Result<usize, rusqlite::Error> {
    let con = get_conn()?;
    con.execute("INSERT INTO sessions(token, user, created) VALUES(?, ?, ?)", [ token, user.to_string(), date_now_string() ])
}

pub fn clear_all_by_user(user: u16) -> Result<usize, rusqlite::Error> {
    let con = get_conn()?;
    con.execute("DELETE FROM sessions WHERE user = ?", [ user ])
}

pub fn is_valid_session(token: String) -> bool {
    let con = match get_conn() {
        Ok(c) => c,
        Err(_) => return false
    };

    match con.query_row("SELECT * FROM sessions WHERE token = ?", [token], |row| { 
        let t: String = row.get(0)?;
        Ok(t)
     }) {
        Ok(_) => true,
        Err(_) => false
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub flag: u16,
    pub id: u16
}

fn user_struct(row: &rusqlite::Row<'_>) -> Result<User, rusqlite::Error> {
    Ok(User {
        username: row.get(0)?,
        email: row.get(1)?,
        flag: row.get(2)?,
        id: row.get(3)?
    })
}

fn hash_password(psw: &String) -> Result<String,argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(psw.as_bytes(), &salt)?.to_string())
}

pub fn new_user(email: String, username: String, password: String, flag: u16) -> Option<bool> {
    let con = get_conn().ok()?;

    let hash = hash_password( &password );
    if !hash.is_ok() { return None }

    match con.execute("INSERT INTO users (email, username, password, flag) VALUES (?, ?, ?, ?) ", [ email, username, hash.unwrap(), flag.to_string() ]) {
        Ok(_) => return Some(true), // User created! Might instead resolve to login page
        Err(_) => None
    }
}

pub fn get_users() -> Result<Vec<User>, rusqlite::Error> {
    let con = get_conn()?;

    let mut statement = match con.prepare("SELECT username, email, flag, id FROM users") {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    
    let user_iter = match statement.query_map([], |row| user_struct(row)) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };

    let mut users: Vec<User> = Vec::new();

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

pub fn get_user(id: String) -> Option<User> {
    let con = get_conn().ok()?;
    match con.query_row("SELECT username, email, flag, id FROM users WHERE email = ?", [id], |row| user_struct(row)) {
        Ok(u) => Some(u),
        Err(_) => None
    }
}

pub fn delete_user(id: u16) -> Result<bool, rusqlite::Error> {
    let con = get_conn()?;
    match clear_all_by_user(id) {
        Ok(_) => { },
        Err(e) => print!("failed to clear user({}) session: {}", id, e.to_string())
    }
    match con.execute("DELETE FROM users WHERE id = ?", [id]) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

pub fn set_flag(id: u16, flag: u16) -> Result<bool, rusqlite::Error> {
    let con = get_conn()?;
    match clear_all_by_user(id) {
        Ok(_) => { },
        Err(e) => print!("failed to clear user({}) session: {}", id, e.to_string())
    }
    match con.execute("UPDATE users SET flag = ? WHERE id = ?", [flag, id]) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

