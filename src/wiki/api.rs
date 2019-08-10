use crate::http::*;
use crate::wiki::types::Wiki;
use scraper::{Html, Selector};

//scrapes from xml data returned from wikipedia api
//forced to use xml as json representation of the data by the api is really unpredictable
pub fn get_wiki_summary(keyword: &str) -> Result<Wiki, Box<dyn std::error::Error>> {
    let keyword = keyword.replace(" ", "%20");
    let url = format!(
        "https://en.wikipedia.org/w/api.php?format=xml&action=query&prop=extracts&exintro&explaintext&redirects=1&titles={}",
        keyword.to_ascii_lowercase()
    );

    let request = HttpRequest {
        url,
        method: HttpMethod::Get,
        body: None,
    };

    let html = request.make_request()?;
    let document = Html::parse_document(&html);
    let select_summary = Selector::parse("extract").unwrap();
    let select_title = Selector::parse("page").unwrap();

    let summary = document
        .select(&select_summary)
        .next()
        .unwrap()
        .text()
        .collect();

    let title = document
        .select(&select_title)
        .next()
        .unwrap()
        .value()
        .attr("title")
        .unwrap()
        .to_string();

    Ok(Wiki::new(summary, title))
}
