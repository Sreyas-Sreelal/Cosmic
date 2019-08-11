use std::error;
use std::fmt;

//TorrentPageInfo
//holds information about torrent that's scraped
pub struct TorrentPageInfo {
    pub name: String,
    pub magnet_url: String,
    pub description: String,
    pub url: String,
}
//TorrentClient
//holds proxy links,request methods
pub struct TorrentClient {
    pub proxy_link: String,
}

#[derive(Debug)]
pub struct ProxyNotFoundError;

impl fmt::Display for ProxyNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to fetch proxy links")
    }
}
impl error::Error for ProxyNotFoundError {
    fn description(&self) -> &str {
        "Proxy links not found"
    }
}

#[derive(Debug)]
pub struct EmptyResultError {
    pub item: String,
}

impl fmt::Display for EmptyResultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to fetch result for item {}", &self.item)
    }
}
impl error::Error for EmptyResultError {
    fn description(&self) -> &str {
        "No results available"
    }
}
