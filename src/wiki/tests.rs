#[cfg(test)]
use crate::wiki::api::*;

#[test]
fn test_wiki() {
    let wiki = get_wiki_summary("rUst laNguage");
    assert_eq!(wiki.is_ok(),true);
    let wiki = wiki.unwrap();

    assert_eq!(wiki.summary,"Rust is a multi-paradigm system programming language focused on safety, especially safe concurrency. Rust is syntactically similar to C++, but is designed to provide better memory safety while maintaining high performance.\nRust was originally designed by Graydon Hoare at Mozilla Research, with contributions from Dave Herman, Brendan Eich, and others. The designers refined the language while writing the Servo layout or browser engine and the Rust compiler. The compiler is free and open-source software dual-licensed under the MIT License and Apache License 2.0.\nRust was the \"most loved programming language\" in the Stack Overflow Developer Survey for 2016, 2017, 2018, and 2019.");
    assert_eq!(wiki.title,"Rust (programming language)");
}
