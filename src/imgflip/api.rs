use crate::http::*;
use crate::imgflip::types::*;
use rand::seq::SliceRandom;

//get_memes
//sends request to imgflip api to get a list of memes
//gets a random meme
//returns it
pub fn get_memes() -> Result<Memes, Box<std::error::Error>> {
    let request = HttpRequest {
        url: "https://api.imgflip.com/get_memes".to_string(),
        method: HttpMethod::Get,
        body: None,
    };

    let data = request.make_request()?;
    let memes: GetMemes = serde_json::from_str(&data)?;
    let random_meme = memes
        .data
        .memes
        .choose(&mut rand::thread_rng())
        .cloned()
        .unwrap();

    Ok(random_meme)
}
