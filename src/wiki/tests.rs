#[cfg(test)]
use crate::wiki::api::*;

#[test]
fn test_wiki() {
    let wiki = get_wiki_summary("rUst laNguage");
    assert_eq!(wiki.is_ok(), true);
    let wiki = wiki.unwrap();
    assert_eq!(wiki.summary.starts_with("Rust is a"),true);
    assert_eq!(wiki.title, "Rust (programming language)");
}
