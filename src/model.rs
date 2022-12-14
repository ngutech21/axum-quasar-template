use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub year: i32,
}

impl Movie {
    pub fn load_dummy_data() -> Vec<Movie> {
        let result: Vec<Movie> = serde_json::from_str(include_str!("../dummy_data.json")).unwrap();
        result
    }
}
