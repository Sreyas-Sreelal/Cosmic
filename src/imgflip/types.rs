use serde::Deserialize;

//see https://api.imgflip.com/get_memes
//for type definitions

#[derive(Deserialize, Clone)]
pub struct Memes {
    pub id: String,
    pub name: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub box_count: i32,
}

#[derive(Deserialize)]
pub struct GetMemes {
    pub success: bool,
    pub data: Data,
}
#[derive(Deserialize)]
pub struct Data {
    pub memes: Vec<Memes>,
}
