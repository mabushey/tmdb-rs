# The Movie Database

![The Movie Database](https://www.themoviedb.org/static_cache/v4/logos/408x161-powered-by-rectangle-green-bb4301c10ddc749b4e79463811a68afebeae66ef43d17bcfd8ff0e60ded7ce99.png)

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

## Acknowledgements

* This lib is heavily inspired by [omdb-rs](https://github.com/aldrio/omdb-rs)

https://www.themoviedb.org/