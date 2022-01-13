use crate::user::userutil::{ Token };
use crate::util::{ permissions, ferr };
use crate::content::{ manager };
use rocket::response::status::NoContent;
use serde::{ Deserialize, Serialize };
use rocket::serde::json::Json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    pub parent: Option<String>,
    pub flag: Option<u16>,
    pub position: Option<u16>,
    pub next: Option<String>
}

#[patch("/entity/<id>", data="<input>")]
pub fn edit_entity(user: Token, input: Json<Entity>, id: String) -> Result<NoContent, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    let old: manager::Entity = match manager::get_entity(id.clone()) {
        Some(e) => e,
        None => return Err(ferr::q_err(404, &format!("no entity with id \"{}\" exists.", &id)))
    };

    match manager::edit_entity(&id, input.flag.unwrap_or(old.flag), input.position.unwrap_or(old.position), &input.parent.as_deref().unwrap_or(&old.parent.unwrap_or("root".to_string())).to_string(), &input.next.as_deref().unwrap_or(&old.next.unwrap_or("".to_string())).to_string() ) {
        Ok(_) => Ok(NoContent),
        Err(e) => return Err(ferr::q_err(500, &e.to_string()))
    }
}

#[derive(Deserialize)]
pub struct MetaData {
    pub thumbnail: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub rating: Option<f32>,
    pub age_rating: Option<String>,
    pub language: Option<String>,
    pub year: Option<u16>
}

#[patch("/metadata/<id>", data="<input>")]
pub fn edit_metadata(user: Token, input: Json<MetaData>, id: String) -> Result<NoContent, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    let old: manager::MetaData = match manager::get_metadata(id.clone()) {
        Some(e) => e,
        None => return Err(ferr::q_err(404, &format!("no metadata with id \"{}\" exists.", &id)))
    };

    match manager::edit_metadata(&id, &input.thumbnail.clone().unwrap_or(old.thumbnail), &input.banner.clone().unwrap_or(old.banner), &input.description.clone().unwrap_or(old.description), &input.name.clone().unwrap_or(old.name), input.rating.clone().unwrap_or(old.rating), &input.age_rating.clone().unwrap_or(old.age_rating), &input.language.clone().unwrap_or(old.language), input.year.unwrap_or(old.year)) {
        Ok(_) => Ok(NoContent),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}

#[derive(Deserialize)]
pub struct Source {
    pub parent: Option<String>,
    pub path: Option<String>,
    pub position: Option<u16>
}

#[patch("/source/<id>", data = "<input>")]
pub fn edit_source(user: Token, input: Json<Source>, id: String) -> Result<NoContent, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    let old: manager::Source = match manager::get_source(id.clone()) {
        Some(e) => e,
        None => return Err(ferr::q_err(404, &format!("no source with id \"{}\" exists.", &id)))
    };

    match manager::edit_source(&id, &input.parent.clone().unwrap_or(old.parent), &input.path.clone().unwrap_or(old.path), input.position.unwrap_or(old.position)) {
        Ok(_) => Ok(NoContent),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}