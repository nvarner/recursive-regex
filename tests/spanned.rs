use recursive_regex::{from_regex_tree_and_str, RegexTree, Spanned};
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Play {
    title: Spanned<String>,
    year: Spanned<u32>,
    tags: Vec<Spanned<String>>,
}

#[test]
fn main() {
    let file = "Title: Romeo and Juliet
Year: 1597
Tags: tragedy

This text is unrelated.

Title: Hamilton
Year: 2015
Tags: musical, historical";

    let regex_tree =
        RegexTree::root(r"Title: (?P<title>.*)\nYear: (?P<year>.*)\nTags: (?P<tags>.*)(?:\n|$)")
            .with_child("tags", RegexTree::leaf(r"[a-z]+"))
            .build();

    let plays: Vec<Play> = from_regex_tree_and_str(&regex_tree, file).unwrap();

    let expected = vec![
        Play {
            title: Spanned::new_raw("Romeo and Juliet".to_owned(), 7, 23),
            year: Spanned::new_raw(1597, 30, 34),
            tags: vec![Spanned::new_raw("tragedy".to_owned(), 41, 48)],
        },
        Play {
            title: Spanned::new_raw("Hamilton".to_owned(), 82, 90),
            year: Spanned::new_raw(2015, 97, 101),
            tags: vec![
                Spanned::new_raw("musical".to_owned(), 108, 115),
                Spanned::new_raw("historical".to_owned(), 117, 127),
            ],
        },
    ];

    assert_eq!(expected, plays);
}
