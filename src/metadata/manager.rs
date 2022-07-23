use crate::content::{ manager as content_manager};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Abook {
    pub description: String,
    pub releaseDate: String
}

#[derive(Deserialize, Debug)]
pub struct MainBook {
    pub cover: String,
    pub largeCover: String,
    pub name: String,
    pub grade: f32 // rating?
}

#[derive(Deserialize, Debug)]
pub struct Book {
    pub abook: Option<Abook>,
    pub book: MainBook
}

#[derive(Deserialize, Debug)]
pub struct Storytel {
    pub books: Vec<Book>,
    suggestions: Vec<Book>
}

pub async fn get_audio_metadata(query: String) -> Result<Vec<content_manager::MetaData>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.storytel.com/api/search.action?q=".to_string() + &query).await?.json::<Storytel>().await?;
    let mut meta_list: Vec<content_manager::MetaData> = Vec::new();

    for book in resp.books {
        match book.abook {
            Some(v) => {
                let as_metadata = content_manager::MetaData {
                    parent: "no parental figure?".to_string(),
                    thumbnail: "https://www.storytel.com".to_string() + &book.book.cover,
                    banner: "https://www.storytel.com".to_string() + &book.book.largeCover,
                    description: v.description,
                    name: book.book.name,
                    rating: book.book.grade,
                    age_rating: "i am adolt".to_string(),
                    language: "English".to_string(),
                    year: v.releaseDate[..4].to_string().parse().unwrap_or(1984)
                };

                meta_list.push(as_metadata);
            },
            None => { }
        }
    }

    Ok(meta_list)
}