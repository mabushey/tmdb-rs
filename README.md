# The Movie Database

This is a wrapper around the [TMDb API](https://developers.themoviedb.org/3).  
You can search and fetch Movies from **The Movie Database**.

## Usage

```rust
extern crate tmdb;

use tmdb::model::*;
use tmdb::themoviedb::*;

fn main() {
    let tmdb: TMDb = TMDb { api_key: env!("TMDB_API_KEY") };

    let movie: Movie = tmdb.fetch_movie(157336).unwrap();
    println!("{:?}", movie);

    let search_movies: Vec<SearchMovie> = tmdb.search_movie("Interstellar");

    for movie in search_movies {
        println!("{} {}", movie.title, movie.release_date.unwrap());
    }

}
```

https://www.themoviedb.org/