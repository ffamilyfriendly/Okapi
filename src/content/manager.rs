use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum EntityType {
    Audio,
    Movie,
    Series,
    Category
}

#[derive(Serialize, Deserialize)]
pub enum EntityFlags {
    Private = 1 << 0
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub id: String,
    pub parent: Entity,
    pub path: String,
    pub position: u16
};

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub parent: Entity,
    pub thumbnail: String,
    pub banner: String,
    pub description: String
    pub rating: u8,
    pub age_rating: String,
    pub language: String,
    pub year: u16
}

#[derive(Serialize, Deserialize)]
struct Entity {
    pub id: String,
    pub parent: Option<Entity>,
    pub flag: u16,
    pub entity_type: EntityType,
    pub creator_uid: u16,
    pub position: u16,
    /* other objects that has entity as parent */
    pub sources: Option<Vec<Source>>,
    pub metadata: Option<MetaData>
};

/*
    This task is quite daunting as a lot needs to go right. Also have to figure out how the fuck to treat series.
*/