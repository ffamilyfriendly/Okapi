use serde::{Deserialize, Serialize};
use rusqlite::{ Connection };
use crate::util::gen_id::{gen_id};

#[derive(Serialize, Deserialize, Clone)]
pub enum EntityType {
    Audio,
    Movie,
    Series,
    Category
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_str = match self {
            EntityType::Audio => "Audio",
            EntityType::Movie => "Movie",
            EntityType::Series => "Series",
            EntityType::Category => "Category"
        };

        write!(f, "{}", as_str)
    }
}

impl std::convert::From<&String> for EntityType {
    fn from(s: &String) -> EntityType {
        let l: &str = &s;
        match l {
            "Audio" => EntityType::Audio,
            "Movie" => EntityType::Movie,
            "Series" => EntityType::Series,
            "Category" => EntityType::Category,
            _ => EntityType::Movie
        }
    }
}


#[derive(Serialize, Deserialize)]
pub enum EntityFlags {
    Private = 1 << 0
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub parent: String,
    pub path: String,
    pub position: u16
}

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub parent: String,
    pub thumbnail: String,
    pub banner: String,
    pub description: String,
    pub name: String,
    pub rating: f32,
    pub age_rating: String,
    pub language: String,
    pub year: u16
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub parent: Option<String>,
    pub flag: u16,
    pub entity_type: EntityType,
    pub creator_uid: u16,
    pub position: u16,
    /* other objects that has entity as parent */
    pub sources: Option<Vec<Source>>,
    pub metadata: Option<MetaData>,
    pub next: Option<String>
}

fn get_conn() -> Result<Connection, rusqlite::Error> {
    Connection::open("data.sqlite")
}

/*
    This task is quite daunting as a lot needs to go right. Also have to figure out how the fuck to treat series.
*/

// GET

fn get_source_struct(row: &rusqlite::Row<'_>) -> Result<Source, rusqlite::Error> {
    Ok(
        Source {
            id: row.get(0)?,
            parent: row.get(1)?,
            path: row.get(2)?,
            position: row.get(3)?
        }
    )
}

fn get_metadata_struct(row: &rusqlite::Row<'_>) -> Result<MetaData, rusqlite::Error> {
    Ok(
        MetaData {
            parent: row.get(0)?,
            thumbnail: row.get(1)?,
            banner: row.get(2)?,
            description: row.get(3)?,
            name: row.get(4)?,
            rating: row.get(5)?,
            age_rating: row.get(6)?,
            language: row.get(7)?,
            year: row.get(8)?
        }
    )
}

fn get_entity_struct(row: &rusqlite::Row<'_>) -> Result<Entity, rusqlite::Error> {
    Ok(
        Entity {
            id: row.get(0)?,
            parent: row.get(1)?,
            next: row.get(2)?,
            flag: row.get(3)?,
            //           #       This whole part is extremely haram        #
            //           vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            entity_type: (&row.get(4).unwrap_or("Movie".to_string())).into(),
            creator_uid: row.get(5)?,
            position: row.get(6)?,
            metadata: None,
            sources: None
        }
    )
}

pub fn get_metadata(id: String) -> Option<MetaData> {
    let con = match get_conn() {
        Ok(c) => c,
        Err(_) => return None
    };

    match con.query_row("SELECT * FROM metadata WHERE id = ?", [id], |row| get_metadata_struct(row)) {
        Ok(e) => Some(e),
        Err(_) => None
    }
}

pub fn get_sources(id: String) -> Option<Vec<Source>> {
    let con = match get_conn() {
        Ok(c) => c,
        Err(_) => return None
    };

    let mut statement = match con.prepare("SELECT * FROM sources WHERE parent = ?") {
        Ok(r) => r,
        Err(_) => return None
    };
    
    let source_iter = match statement.query_map([id], |row| get_source_struct(row)) {
        Ok(r) => r,
        Err(_) => return None
    };
    
    let mut sources: Vec<Source> = Vec::new();

    for source in source_iter {
        match source {
            Ok(s) => {
                sources.push(s)
            },
            Err(_) => {  }
        };
    }

    return Some(sources);
}

pub fn get_public_entity(id: String) -> Option<Entity> {
    let con = match get_conn() {
        Ok(c) => c,
        Err(_) => return None
    };

    match con.query_row("SELECT * FROM entities WHERE id = ? AND flag & 1 << 0 != 1 << 0", [id], |row| get_entity_struct(row)) {
        Ok(e) => Some(e),
        Err(_) => None
    }
}

pub fn get_entity(id: String) -> Option<Entity> {
    let con = match get_conn() {
        Ok(c) => c,
        Err(_) => return None
    };

    match con.query_row("SELECT * FROM entities WHERE id = ?", [id], |row| get_entity_struct(row)) {
        Ok(e) => Some(e),
        Err(_) => None
    }
}

pub fn get_collection(id: String, public: bool) -> Option<Entity> {
    let mut ent = match public {
        true => get_public_entity(id.clone())?,
        false => get_entity(id.clone())?
    };

    ent.sources = get_sources(id.clone());
    ent.metadata = get_metadata(id.clone());
    Some(ent)
}

// DELETE
fn generic_delete(table: &str, id: &str) -> Result<bool, rusqlite::Error> {
    let con = get_conn()?;
    con.execute("DELETE FROM ? WHERE id = ?", [table, id])?;
    Ok(true)
}

pub fn delete_metadata(id: &str) -> Result<bool, rusqlite::Error> {
    Ok(generic_delete("metadata", id)?)
}

pub fn delete_sources(id: &str) -> Result<bool, rusqlite::Error> {
    let con = get_conn()?;
    con.execute("DELETE FROM sources WHERE parent = ?", [id])?;
    Ok(true)
}

pub fn delete_source(id: &str) -> Result<bool, rusqlite::Error> {
    Ok(generic_delete("sources", id)?)
}

pub fn delete_entity(id: &str) -> Result<bool, rusqlite::Error> {
    Ok(generic_delete("entities", id)?)
}

// GENERATE
pub fn generate_metadata(parent: &str, thumbnail: &str, banner: &str, description: &str, name: &str, rating: f32, age_rating: &str, language: &str, year: u16) -> Result<bool, rusqlite::Error> {
    let con = get_conn()?;
    match con.execute("INSERT INTO metadata VALUES(?,?,?,?,?,?,?,?,?)", [parent, thumbnail, banner, description, name, &rating.to_string(), age_rating, language, &year.to_string()]) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

pub fn generate_source(parent: &str, path: &str, position: Option<u16>) -> Result<bool, rusqlite::Error> {
    let con = get_conn()?;
    let id: String = gen_id();
    match con.execute("INSERT INTO sources VALUES(?,?,?,?)", [&id, parent, path, &position.unwrap_or(0).to_string()]) {
        Ok(_) => Ok(true),
        Err(e) => Err(e)
    }
}

pub fn generate_entity(flag: u16, entity_type: EntityType, creator_uid: u16, position: Option<u16>, parent: Option<String>, next: Option<String> ) -> Result<Entity, rusqlite::Error> {
    let con = get_conn()?;
    let id: String = gen_id();
    match con.execute("INSERT INTO entities VALUES(?,?,?,?,?,?,?)", [&id, parent.as_deref().unwrap_or(&"".to_string()), &next.as_deref().unwrap_or(&"".to_string()), &flag.to_string(), &entity_type.to_string(), &creator_uid.to_string(), &position.unwrap_or(0).to_string()]) {
        Ok(_) => {
            Ok(
                Entity {
                    id: id,
                    parent: parent,
                    flag: flag,
                    entity_type: entity_type,
                    creator_uid: creator_uid,
                    position: position.unwrap_or(0),
                    sources: None,
                    metadata: None,
                    next: next
                }
            )
        },
        Err(e) => Err(e)
    }
}