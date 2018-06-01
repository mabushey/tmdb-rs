use model::{Movie,SearchMovie,SearchResult,FindResult};
use reqwest;

const BASE_URL: &'static str = "https://api.themoviedb.org/3";
// const BASE_IMG_URL: &'static str = "https://image.tmdb.org/t/p/w500";
// "https://image.tmdb.org/t/p/w700_and_h392_bestv2/gq4Z1pfOWHn3FKFNutlDCySps9C.jpg"

pub trait Executable<T> {
    fn execute(&self) -> Result<T, reqwest::Error>;
}

pub trait Search<'a> {
    fn title(&mut self, title: &'a str) -> &mut SearchData<'a>;
    fn year(&mut self, year: u64) -> &mut SearchData<'a>;
}

#[derive(Debug)]
pub struct SearchData<'a> {
    pub tmdb: TMDb,
    pub title: Option<&'a str>,
    pub year: Option<u64>,
}

impl<'a> Search<'a> for SearchData<'a> {
    
    fn title(&mut self, title: &'a str) -> &mut SearchData<'a> {
        self.title = Some(title);
        return self;
    }

    fn year(&mut self, year: u64) -> &mut SearchData<'a> {
        self.year = Some(year);
        return self;
    }
}

impl<'a> Executable<SearchResult> for SearchData<'a> {
    
    fn execute(&self) -> Result<SearchResult, reqwest::Error> {
        let url: String = match self.year {
            None => format!("{}/search/movie?api_key={}&language={}&query={}&append_to_response=images",
                            BASE_URL, self.tmdb.api_key, self.tmdb.language, self.title.unwrap()),
            Some(year) => format!("{}/search/movie?api_key={}&language={}&query={}&year={}&append_to_response=images",
                            BASE_URL, self.tmdb.api_key, self.tmdb.language, self.title.unwrap(), year),
        };

        return reqwest::get(&url)?.json();
    }
}

pub enum Appendable {
    Videos,
    Credits,
}

pub trait Fetch {
    fn id(&mut self, id: u64) -> &mut FetchData;
    fn append_videos(&mut self) -> &mut FetchData;
    fn append_credits(&mut self) -> &mut FetchData;
}

pub struct FetchData {
    pub tmdb: TMDb,
    pub id: Option<u64>,
    pub append_to_response: Vec<Appendable>,
}

impl Fetch for FetchData {
    fn id(&mut self, id: u64) -> &mut FetchData {
        self.id = Some(id);
        return self;
    }

    fn append_videos(&mut self) -> &mut FetchData {
        self.append_to_response.push(Appendable::Videos);
        return self;
    }

    fn append_credits(&mut self) -> &mut FetchData {
        self.append_to_response.push(Appendable::Credits);
        return self;
    }
}

impl Executable<Movie> for FetchData {
    fn execute(&self) -> Result<Movie, reqwest::Error> {
        let mut url: String = format!("{}/movie/{}?api_key={}&language={}",
                                  BASE_URL, self.id.unwrap(), self.tmdb.api_key, self.tmdb.language);


        if self.append_to_response.len() != 0 {
            url.push_str("&append_to_response=");
            for appendable in &self.append_to_response {
                match appendable {
                    Appendable::Videos => url.push_str("videos,"),
                    Appendable::Credits => url.push_str("credits,"),
                }
            }
        }

        return reqwest::get(&url)?.json();
    }
}

pub trait Find<'a> {
    fn imdb_id(&mut self, imdb_id: &'a str) -> &mut FindData<'a>;
}

pub struct FindData<'a> {
    pub tmdb: TMDb,
    pub imdb_id: Option<&'a str>,
}

impl<'a> Find<'a> for FindData<'a> {

    fn imdb_id(&mut self, imdb_id: &'a str) -> &mut FindData<'a> {
        self.imdb_id = Some(imdb_id);
        return self;
    }
}

impl<'a> Executable<FindResult> for FindData<'a> {
    fn execute(&self) -> Result<FindResult, reqwest::Error> {
        let url = format!("{}/find/{}?api_key={}&external_source=imdb_id&language={}&append_to_response=images", BASE_URL, self.imdb_id.unwrap(), self.tmdb.api_key, self.tmdb.language);
        return reqwest::get(&url)?.json();
    }
}

pub trait TMDbApi {
    fn search(&self) -> SearchData;
    fn fetch(&self) -> FetchData;
    fn find(&self) -> FindData;
}

#[derive(Debug,Clone)]
pub struct TMDb { 
    pub api_key: &'static str,
    pub language: &'static str,
}

impl TMDbApi for TMDb {

    fn search(&self) -> SearchData {
        let tmdb = self.clone();
        return SearchData {tmdb: tmdb, title: None, year: None};
    }

    fn fetch(&self) -> FetchData {
        let tmdb = self.clone();
        return FetchData {tmdb: tmdb, id: None, append_to_response: vec![]};
    }

    fn find(&self) -> FindData {
        let tmdb = self.clone();
        return FindData {tmdb: tmdb, imdb_id: None};
    }
    
}

pub trait Fetchable {
    fn fetch(&self, tmdb: &TMDb) -> Result<Movie, reqwest::Error>;
}

impl Fetchable for SearchMovie {
    fn fetch(&self, tmdb: &TMDb) -> Result<Movie, reqwest::Error> {
        return tmdb.fetch()
            .id(self.id)
            .execute();
    }
}
