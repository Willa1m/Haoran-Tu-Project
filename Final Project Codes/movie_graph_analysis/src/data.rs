use serde::Deserialize;
use csv::Reader;
use std::fs::File;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Movie {
    pub movie_id: u32,
    pub title: String,
    pub genres: String,
}

#[derive(Debug, Deserialize)]
pub struct Rating {
    pub user_id: u32,
    pub movie_id: u32,
    pub rating: f32,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub user_id: u32,
    pub movie_id: u32,
    pub tag: String,
    pub timestamp: u64,
}

pub fn load_movies(filepath: &str) -> Result<Vec<Movie>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut rdr = Reader::from_reader(file);
    let mut movies = Vec::new();

    for result in rdr.deserialize() {
        let movie: Movie = result?;
        movies.push(movie);
    }
    Ok(movies)
}

pub fn load_ratings(filepath: &str) -> Result<Vec<Rating>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut rdr = Reader::from_reader(file);
    let mut ratings = Vec::new();

    for result in rdr.deserialize() {
        let rating: Rating = result?;
        ratings.push(rating);
    }
    Ok(ratings)
}

pub fn load_tags(filepath: &str) -> Result<Vec<Tag>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut rdr = Reader::from_reader(file);
    let mut tags = Vec::new();

    for result in rdr.deserialize() {
        let tag: Tag = result?;
        tags.push(tag);
    }
    Ok(tags)
}
