# The Movie Databse

This is a wrapper around the [TMDb API](https://developers.themoviedb.org/3).  
You can search and fetch Movies from **The Movie Databse**.

## Usage

```rust
let tmdb: TMDb = TMDb { api_key: "123456789" };

let movie = tmdb.fetch_movie(157336).unwrap();
println!("{:?}", movie);

let search_movies = tmdb.search_movie("Interstellar").unwrap();
println!("{:?}", search_movies);
```

https://www.themoviedb.org/