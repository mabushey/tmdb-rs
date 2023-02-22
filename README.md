# The Movie Database

![The Movie Database](https://www.themoviedb.org/assets/2/v4/logos/408x161-powered-by-rectangle-green-bb4301c10ddc749b4e79463811a68afebeae66ef43d17bcfd8ff0e60ded7ce99.png)

This is a wrapper around the [TMDb API](https://developers.themoviedb.org/3).
This is a fork, the parent repo is [tmdb-rs](https://gitlab.com/Cir0X/tmdb-rs.git)

## Usage

```rust
extern crate tmdb;

use tmdb::model::*;
use tmdb::themoviedb::*;

fn main() {
    let tmdb = TMDb { api_key: env!("TMDB_API_KEY"), language: "en" };

    let movies = tmdb.search()
        .title("Interstellar")
        .year(2014)
        .execute()
        .unwrap();

    let id = movies.results[0].id;
    
    let interstellar: Movie = tmdb.fetch()
        .id(id)
        .execute()
        .unwrap();

    println!("{:#?}", interstellar);
}
```

## Actions

Currently there are 3 actions available:

* Searching
* Fetching
* Finding

### Searching

You can search for movies by `title` and `year`.

```rust
let page = tmdb.search()
    .title("Bicentennial Man")
    .year(1999)
    .execute()
    .unwrap();

let movies = page.results;
```

### Fetching

You can fetch a movie, when you know its ID. Then you get all the movie details.

```rust
let movie = tmdb.fetch()
    .id(157336)
    .execute()
    .unwrap();
```

When you don't have any movie ID, you can search for a movie and then easily fetch the full details.

```rust
let page = tmdb.search()
   .title("Bicentennial Man")
   .year(1999)
   .execute()
   .unwrap();

let movies = page.results;
let movie = movies[0].fetch(&tmdb).unwrap();
```

Furthermore you can request some more data with the [append to response](https://developers.themoviedb.org/3/getting-started/append-to-response) feature.

```rust
let movie = tmdb.fetch()
    .id(2277)
    .append_videos()
    .append_credits()
    .execute()
    .unwrap();
```

### Finding

[Finding](https://developers.themoviedb.org/3/find/find-by-id) a movie with an external ID is currently supported with IMDB IDs.

```rust
let find_result = tmdb.find()
    .imdb_id("tt0816692")
    .execute()
    .unwrap();

let movies = find_result.movie_results;
```

### Dynamic TMDB_API_KEY

To use an API key that's not hardcoded or an ENV, use the lazy_static crate.
```rust
lazy_static! {
  static ref CONFIG: Cfg = read_config();
}

let tmdb = TMDb { api_key: &CONFIG.tmdb_api_key, language: "en" };
```

read_config uses serde_yaml to read in a config file containing
`tmdb_api_key: [redacted]`.

## Acknowledgements

* This lib is heavily inspired by [omdb-rs](https://github.com/aldrio/omdb-rs)
* [The Movie Database (TMDb)](https://www.themoviedb.org/)
