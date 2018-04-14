#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Genre {
    pub id: u64,
    name: String,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Movie {
    pub id: u64,
    pub imdb_id: String,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: Option<String>, // ToDo: Date Type
    pub runtime: u8,
    pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub budget: u64,
    pub adult: bool,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct SearchMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: Option<String>, // ToDo: Date Type
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub adult: bool,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct SearchResult {
    pub page: u8,
    pub total_results: u8,
    pub total_pages: u8,
    pub results: Vec<SearchMovie>,
}