#[cfg(test)]
use crate::torrent::types::*;

#[test]
fn test_torrent() {
    let mut client = TorrentClient {
        proxy_link: String::new(),
    };

    let pageinfo = client.get_torrent_info("games");
    assert_eq!(pageinfo.is_ok(), true);

    let pageinfo = pageinfo.unwrap();

    assert_eq!(pageinfo.name.is_empty(),false);
    assert_eq!(pageinfo.magnet_url.starts_with("magnet:?xt="),true);
    assert_eq!(pageinfo.url.starts_with("https://"),true);
}
