use std::collections::HashMap;

use recursive_regex::{from_regex_tree_and_str, RegexTree};
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Person<'a> {
    pub name: &'a str,
    pub favorite_numbers: Vec<i32>,
}

#[test]
fn main() {
    let file = "Lina's favorite number is 2
Selah's favorite numbers are 3, 6, 8, and 12
Aili's favorite numbers are 1, 4, 10, and 13
Gemma's favorite numbers are 9, 10, 11, and 14
Genoveva's favorite numbers are 2, 10, 11, 13, and 15
Eryk's favorite number is 6
Alpheus's favorite numbers are 1, 6, 12, and 19
Sven's favorite numbers are 1 and 5
Annabella's favorite numbers are 2, 6, and 14";

    let regex_subtree = RegexTree::new(Regex::new("\\d+").unwrap(), HashMap::new());
    let mut hash_map = HashMap::new();
    hash_map.insert("favorite_numbers".to_string(), regex_subtree);

    let regex_tree = RegexTree::new(
        Regex::new("(?P<name>.*)'s favorite numbers? (?:is|are) (?P<favorite_numbers>.*)").unwrap(),
        hash_map,
    );

    let people: Vec<Person> = from_regex_tree_and_str(&regex_tree, file).unwrap();

    let expected_people = vec![
        Person {
            name: "Lina",
            favorite_numbers: vec![2],
        },
        Person {
            name: "Selah",
            favorite_numbers: vec![3, 6, 8, 12],
        },
        Person {
            name: "Aili",
            favorite_numbers: vec![1, 4, 10, 13],
        },
        Person {
            name: "Gemma",
            favorite_numbers: vec![9, 10, 11, 14],
        },
        Person {
            name: "Genoveva",
            favorite_numbers: vec![2, 10, 11, 13, 15],
        },
        Person {
            name: "Eryk",
            favorite_numbers: vec![6],
        },
        Person {
            name: "Alpheus",
            favorite_numbers: vec![1, 6, 12, 19],
        },
        Person {
            name: "Sven",
            favorite_numbers: vec![1, 5],
        },
        Person {
            name: "Annabella",
            favorite_numbers: vec![2, 6, 14],
        },
    ];
    assert_eq!(expected_people, people);
}
