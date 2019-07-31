#[cfg(test)]
use crate::torrent::types::*;

#[test]
fn test_torrent() {
    let mut client = TorrentClient {
        proxy_link: String::new(),
    };

    let pageinfo = client.get_torrent_info("witcher");
    assert_eq!(pageinfo.is_ok(), true);

    let pageinfo = pageinfo.unwrap();

    assert_eq!(
        pageinfo.name,
        "The Witcher 3 Wild Hunt Game of the Year Edition PROPER-GOG"
    );
    assert_eq!(pageinfo.magnet_url, "magnet:?xt=urn:btih:f6921cf841c1d8a6b1233eac6034303e6f40f4b5&dn=The+Witcher+3+Wild+Hunt+Game+of+the+Year+Edition+PROPER-GOG&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.openbittorrent.com%3A80&tr=udp%3A%2F%2Fopen.demonii.com%3A1337&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Fexodus.desync.com%3A6969");
    assert_eq!(pageinfo.url, "https://proxtpb.art/torrent/15728783/The_Witcher_3_Wild_Hunt_Game_of_the_Year_Edition_PROPER-GOG");
}
