extern crate tmdb;

use tmdb::themoviedb::*;

fn main() {
    let tmdb = TMDb {
        api_key: env!("TMDB_API_KEY"),
        language: "en",
    };

    let find_result = tmdb.find().imdb_id("tt0816692").execute().unwrap();

    let movies = find_result.movie_results;

    println!("Movies: {:#?}", movies);
}
