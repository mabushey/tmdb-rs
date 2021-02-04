extern crate tmdb;

use tmdb::model::*;
use tmdb::themoviedb::*;

fn main() {
    let tmdb = TMDb {
        api_key: env!("TMDB_API_KEY"),
        language: "en",
    };

    let movie: Movie = tmdb
        .fetch()
        .id(2277)
        .append_videos()
        .append_credits()
        .execute()
        .unwrap();

    println!("{:#?}", movie);
}
