#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;

pub mod model;
pub mod themoviedb;

// #[cfg(test)]
// mod unit_tests {

//     use model::*;
//     use themoviedb::*;

//     const API_KEY: &'static str = "123456789";
//     const LANGUAGE: &'static str = "en";
//     // const TMDB: TMDb = TMDb { api_key: API_KEY, language: LANGUAGE};

//     #[test]
//     fn test() {
//         // let result = TMDB.search()
//         //     .title("Interstellar")
//         //     .execute();

//         // println!("{:?}", result);
//         assert_eq!(1,1);
//     }
    
// }

#[cfg(test)]
mod integration_tests {

    use model::{Movie,TV,SearchMovie,FindMovie};
    use themoviedb::*;

    const API_KEY: &'static str = env!("TMDB_API_KEY");
    const LANGUAGE: &'static str = "en";
    const TMDB: TMDb = TMDb { api_key: API_KEY, language: LANGUAGE };

    #[test]
    fn fetch_movie() {
        let movie: Movie = TMDB.fetch()
            .id(157336)
            .execute()
            .unwrap();
        
        assert_eq!("Interstellar", movie.original_title);
    }

    #[test]
    fn fetch_movie_languages() {
        let tmdb_en = TMDb { api_key: API_KEY, language: "en" };
        let movie_en: Movie = tmdb_en.fetch().id(2277).execute().unwrap();
        assert_eq!("Bicentennial Man", movie_en.title);

        let tmdb_de = TMDb { api_key: API_KEY, language: "de" };
        let movie_de: Movie = tmdb_de.fetch().id(2277).execute().unwrap();
        assert_eq!("Der 200 Jahre Mann", movie_de.title);

        let tmdb_es = TMDb { api_key: API_KEY, language: "es" };
        let movie_es: Movie = tmdb_es.fetch().id(2277).execute().unwrap();
        assert_eq!("El hombre bicentenario", movie_es.title);
    }

    #[test]
    fn fetch_movie_append_to_response() {
        let movie: Movie = TMDB.fetch()
            .id(2277)
            .append_videos()
            .append_credits()
            .execute()
            .unwrap();

        assert_eq!(true, movie.videos.is_some());
        assert_eq!(true, movie.credits.is_some());
    }

    #[test]
    fn search_movie() {
        let empty_movies: Vec<SearchMovie> = vec![];
        
        let page = TMDB.search()
            .title("Bicentennial Man")
            .year(1999)
            .execute()
            .unwrap();

        let movies = page.results;
        
        assert_eq!(1, page.total_results);
        assert_ne!(empty_movies, movies);
        assert_eq!("Bicentennial Man", movies[0].title);
    }

    #[test]
    fn find_movie_by_imdb_id() {
        let empty_movies: Vec<FindMovie> = vec![];

        let find_result = TMDB.find()
            .imdb_id("tt0816692")
            .execute()
            .unwrap();

        let movies = find_result.movie_results;

        assert_eq!(1, movies.len());
        assert_ne!(empty_movies, movies);
        assert_eq!("Interstellar", movies[0].title);
    }

    #[test]
    fn fetch_searched_movie() {
        let page = TMDB.search()
            .title("Bicentennial Man")
            .year(1999)
            .execute()
            .unwrap();

        let movies = page.results;
        let movie = movies[0].fetch(&TMDB).unwrap();

        assert_eq!(2277, movie.id);
    }

    #[test]
    fn fetch_tv() {
        let tv: TV = TMDB.fetch()
            .id(2316)
            .execute()
            .unwrap();
        
        assert_eq!("The Office", tv.original_name);
    }

    #[test]
    fn fetch_tv_append_to_response() {
        let tv: TV = TMDB.fetch()
            .id(2316)
            .append_videos()
            .append_credits()
            .execute()
            .unwrap();

        assert_eq!(true, tv.videos.is_some());
        assert_eq!(true, tv.credits.is_some());
    }

}
