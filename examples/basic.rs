extern crate tmdb;

use tmdb::model::*;
use tmdb::themoviedb::*;

fn main() {
    let tmdb = TMDb {
        api_key: env!("TMDB_API_KEY"),
        language: "en",
    };

    let movies = tmdb
        .search()
        .title("Interstellar")
        .year(2014)
        .execute()
        .unwrap();

    let id = movies.results[0].id;

    let interstellar: Movie = tmdb.fetch().id(id).execute().unwrap();

    println!("{:#?}", interstellar);
}
