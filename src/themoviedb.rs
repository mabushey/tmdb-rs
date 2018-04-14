use model::{Movie,SearchResult,SearchMovie};
use reqwest;

const BASE_URL: &'static str = "https://api.themoviedb.org/3";
// const BASE_IMG_URL: &'static str = "https://image.tmdb.org/t/p/w500";
// "https://image.tmdb.org/t/p/w700_and_h392_bestv2/gq4Z1pfOWHn3FKFNutlDCySps9C.jpg"

pub trait TMDbApi {
    fn fetch_movie(&self, id: u64) -> Option<Movie>;
    fn search_movie(&self, query: &str) -> Vec<SearchMovie>;
}

pub struct TMDb { 
    pub api_key: &'static str,
    pub language: &'static str,
}

impl TMDbApi for TMDb {

    fn fetch_movie(&self, id: u64) -> Option<Movie> {
        let url: String = format!("{}/movie/{}?api_key={}&language={}", BASE_URL, id, self.api_key, self.language);
        let movie: Movie = reqwest::get(&url).unwrap().json().unwrap();
        return Some(movie);
    }

    fn search_movie(&self, query: &str) -> Vec<SearchMovie> {
        let url: String = format!("{}/search/movie?api_key={}&language={}&query={}&append_to_response=images", BASE_URL, self.api_key, self.language, query);
        let search_result: SearchResult = reqwest::get(&url).unwrap().json().unwrap();
        let movies: Vec<SearchMovie> = search_result.results;
        return movies;
    }

}

pub trait Fetchable {
    fn fetch(&self, tmdb: &TMDb) -> Movie;
}

impl Fetchable for SearchMovie {

    fn fetch(&self, tmdb: &TMDb) -> Movie {
        return tmdb.fetch_movie(self.id).unwrap();
    }

}