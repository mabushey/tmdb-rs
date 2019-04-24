#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Results<T> {
    pub results: Vec<T>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Video {
    pub id: String,
    pub iso_639_1: String,
    pub key: String,
    pub name: String,
    pub site: String,
    pub size: u16,
    #[serde(rename = "type")]
    pub video_type: String,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Cast {
    pub id: u64,
    pub cast_id: u64,
    pub credit_id: String,
    pub character: String,
    pub gender: Option<u8>,
    pub name: String,
    pub profile_path: Option<String>,
    pub order: u8,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Crew {
    pub credit_id: String,
    pub department: String,
    pub gender: Option<u8>,
    pub id: u64,
    pub job: String,
    pub name: String,
    pub profile_path: Option<String>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Credits {
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct Movie {
    pub id: u64,
    pub imdb_id: String,
    pub title: String,
    pub tagline: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub runtime: u32,
    pub homepage: Option<String>,
    pub genres: Vec<Genre>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub budget: u64,
    pub adult: bool,
    pub videos: Option<Results<Video>>,
    pub credits: Option<Credits>,
}

// struct Foo {
//     movie: Movie,
//     seen_by: Vec<User>,
//     // custom
//     location: String,
//     quality: String,
//     version: String,

// }

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct SearchMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub popularity: f64,
    pub adult: bool,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct FindMovie {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub genre_ids: Vec<u16>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub adult: bool,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct SearchResult {
    pub page: u8,
    pub total_results: u8,
    pub total_pages: u8,
    pub results: Vec<SearchMovie>,
}

#[derive(Debug,PartialEq,Deserialize,Serialize)]
pub struct FindResult {
    pub movie_results: Vec<FindMovie>,
}

