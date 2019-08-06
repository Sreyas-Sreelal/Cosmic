use crate::http::{HttpMethod, HttpRequest};
use crate::torrent::types::*;
use log::debug;
use scraper::{Html, Selector};

impl TorrentClient {
    //get_proxy()
    //grabs piratebay proxies from https://piratebay-proxylist.se/
    //returns working proxy url
    fn get_proxy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let request = HttpRequest {
            url: "https://piratebay-proxylist.se/".to_string(),
            method: HttpMethod::Get,
            body: None,
        };

        let html = request.make_request()?;
        let document = Html::parse_document(&html);
        let selector = Selector::parse(".url").unwrap();

        let links = document.select(&selector);
        for link in links {
            if let Some(href) = link.value().attr("data-href") {
                debug!("trying {}", href);
                if self.check_connection(href).is_ok() {
                    self.proxy_link = href.to_string();
                    return Ok(());
                }
            }
        }

        Err(Box::new(ProxyNotFoundError))
    }

    //check_connection
    //checks the connection status of proxies
    fn check_connection(&self, link: &str) -> Result<(), Box<dyn std::error::Error>> {
        let request = HttpRequest {
            url: link.to_string(),
            method: HttpMethod::Get,
            body: None,
        };
        request.make_request()?;

        Ok(())
    }

    //get_torrent_url
    //gets torrent url from the list of search results
    //always prefers first result
    fn get_torrent_url(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let end_point = format!("{}/s/?q={}&page=0&orderby=99", &self.proxy_link, input);
        let request = HttpRequest {
            url: end_point.to_string(),
            method: HttpMethod::Get,
            body: None,
        };
        let html = request.make_request()?;

        let selector = Selector::parse(".detName>a").unwrap();
        let document = Html::parse_document(&html);
        if let Some(element) = document.select(&selector).nth(0) {
            if let Some(url) = element.value().attr("href") {
                return Ok((self.proxy_link.clone() + url).to_string());
            };
        };

        Err(Box::new(EmptyResultError {
            item: input.to_string(),
        }))
    }

    //get_torrent_name
    //fetches page title
    fn get_torrent_name(&self, html: &Html) -> Result<String, Box<dyn std::error::Error>> {
        let selector = Selector::parse("#title").unwrap();
        let element = html.select(&selector).nth(0).unwrap();
        Ok(element.text().collect::<String>().trim().to_string())
    }

    //get_torrent_magnet
    //fetches magnet url from the page
    fn get_torrent_magnet(&self, html: &Html) -> Result<String, Box<dyn std::error::Error>> {
        let selector = Selector::parse(".download>a").unwrap();
        let element = html.select(&selector).nth(0).unwrap();
        Ok(element.value().attr("href").unwrap().to_string())
    }

    //get_torrent_description
    //fetches description of the torrent from the page
    fn get_torrent_description(&self, html: &Html) -> Result<String, Box<dyn std::error::Error>> {
        let selector = Selector::parse(".nfo").unwrap();
        let element = html.select(&selector).nth(0).unwrap();
        Ok(element.text().collect())
    }

    //get_torrent_info
    //main function which search and grabs torrent details
    pub fn get_torrent_info(
        &mut self,
        input: &str,
    ) -> Result<TorrentPageInfo, Box<dyn std::error::Error>> {
        self.get_proxy()?;
        let url = self.get_torrent_url(input)?;
        let request = HttpRequest {
            url: url.to_string(),
            method: HttpMethod::Get,
            body: None,
        };

        let html = request.make_request()?;
        let document = Html::parse_document(&html);
        let name = self.get_torrent_name(&document)?;
        let magnet_url = self.get_torrent_magnet(&document)?;
        let description = self.get_torrent_description(&document)?;

        Ok(TorrentPageInfo {
            name,
            description,
            magnet_url,
            url,
        })
    }
}
