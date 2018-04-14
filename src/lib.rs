#[macro_use]
extern crate serde_derive;
extern crate reqwest;

pub mod model;
pub mod themoviedb;

#[cfg(test)]
mod unit_tests {

    use model::{Movie,SearchMovie};
    use themoviedb::{TMDbApi};

    struct TMDbMock {
        api_key: &'static str
    }

    impl TMDbApi for TMDbMock {

        fn fetch_movie(&self, _: u64) -> Option<Movie> {
            return Some(dummy_movie());
        }

        fn search_movie(&self, _: &str) -> Vec<SearchMovie> {
            return vec![];
        }

    }

    const API_KEY: &'static str = "123456789";
    const TMDB: TMDbMock = TMDbMock { api_key: API_KEY };

    #[test]
    fn api_key_not_modified() {
        assert_eq!(API_KEY, TMDB.api_key);
    }

    #[test]
    fn fetch_movie() {
        let dummy_movie = dummy_movie();
        let movie = TMDB.fetch_movie(157336).unwrap();
        assert_eq!(dummy_movie, movie);
    }

    #[test]
    fn search_movie() {
        let movies = TMDB.search_movie("Interstellar");
        assert_eq!(movies, vec![]);
    }


    fn dummy_movie() -> Movie {
        return Movie {
            id: 157336,
            imdb_id: "ttxxxxxxx".to_string(),
            title: "Interstellar".to_string(),
            original_title: "Interstellar".to_string(),
            original_language: "en".to_string(),
            overview: Some("foo".to_string()),
            release_date: Some("2017-02-02".to_string()),
            runtime: 120,
            genres: vec![],
            poster_path: Some("foo".to_string()),
            backdrop_path: Some("foo".to_string()),
            popularity: 0.0,
            budget: 1200000,
            adult: false,
        };
    }

}

#[cfg(test)]
mod integration_tests {

    use model::{Movie,SearchMovie};
    use themoviedb::{TMDbApi, TMDb};

    const API_KEY: &'static str = env!("TMDB_API_KEY");
    const TMDB: TMDb = TMDb { api_key: API_KEY };

    #[test]
    fn fetch_movie() {
        let movie: Movie = TMDB.fetch_movie(157336).unwrap();
        assert_eq!("Interstellar", movie.original_title);
    }

    #[test]
    fn search_movie() {
        let empty_movies: Vec<SearchMovie> = vec![];
        let movies: Vec<SearchMovie> = TMDB.search_movie("Interstellar");
        assert_ne!(empty_movies, movies);
    }

}
