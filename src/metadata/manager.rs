use crate::content::{ manager as content_manager};
use reqwest;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Abook {
    pub description: String,
    pub releaseDate: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
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
    pub books: Vec<Book>
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
                    rating: book.book.grade * 2.0,
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

#[derive(Deserialize, Debug)]
pub struct Movie {
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub overview: String,
    pub release_date: String,
    pub original_title: String,
    pub original_language: String,
    pub vote_average: f32
}

#[derive(Deserialize, Debug)]
pub struct Tmdb {
    pub results: Vec<Movie>
}

pub async fn get_movie_metadata(query: String, api_key: &String) -> Result<Vec<content_manager::MetaData>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.themoviedb.org/3/search/movie?query=".to_string() + &query + "&api_key=" + api_key).await?.json::<Tmdb>().await?;
    let mut meta_list: Vec<content_manager::MetaData> = Vec::new();

    for movie in resp.results {

        fn getYear(rd: String) -> u16 {
            if &rd.chars().count() < &4 {
                return 1984
            }
            rd[..4].to_string().parse().unwrap_or(1337)
        }

        let as_metadata = content_manager::MetaData {
            parent: "yeet".to_string(),
            thumbnail: "https://image.tmdb.org/t/p/w500".to_string() + &movie.poster_path.unwrap_or("".to_string()),
            banner: "https://image.tmdb.org/t/p/w500".to_string() + &movie.backdrop_path.unwrap_or("".to_string()),
            description: movie.overview,
            name: movie.original_title,
            rating: movie.vote_average,
            age_rating: "lol".to_string(),
            language: movie.original_language,
            year: getYear(movie.release_date)
        };

        meta_list.push(as_metadata);
    }

    Ok(meta_list)
}