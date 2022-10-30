# Recursive Regex

The recursive regex algorithm is designed to be a simple, customizable
parser for basic data files. It matches a regular expression to text as many
times as possible, extracting data via capture groups. On each capture
group, it may recurse with a new regular expression to further parse the
results.

The crate is designed to be used for "parsing" unstructured data or structured
data for which it's not worth writing a real parser. In particular, the
motivation is parsing homework questions, with question title, points,
description, solution, etc. from a LaTeX file.

## Basic example
```rust
use recursive_regex::{RegexTree, from_regex_tree_and_str};
use serde::Deserialize;

// Text we want to deseralize, possibly from a file
let text = r#"
1 2 456 true
8 3 -54 false
"#;

// Regex tree that breaks down each line into numbers and boolean, then breaks
// down numbers into a vec. Regex trees can be explicitly constructed, like so,
// or deserialized with the `deserialize-regex-tree` feature.
let regex_tree = RegexTree::root(r"(?P<nums>[-\d\s]*) (?P<yn>true|false)")
    .with_child("nums", RegexTree::leaf(r"-?\d+"))
    .build();

// Struct to deserialize each line into
#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Line {
    nums: Vec<i32>,
    yn: bool,
}

let deserialized: Vec<Line> = from_regex_tree_and_str(&regex_tree, &text).unwrap();
assert_eq!(deserialized, vec![
    Line { nums: vec![1, 2, 456], yn: true },
    Line { nums: vec![8, 3, -54], yn: false },
]);
```

## Uncaptured
When reading data from a large file, it may be important to verify that the
entire file was actually parsed. This is possible via `get_uncaptured`.

Consider the above example, but imagine having forgotten to allow for negative
signs. In a large enough file where negative signs are uncommon, it may be hard
to notice their ommision. However, we can check to see what was not matched by
a top level regex.
```rust
use recursive_regex::{RegexTree, get_uncaptured};
use serde::Deserialize;

let text = r#"
1 2 456 true
8 3 -54 false
"#;

// Note that we do not account for `-` in numbers. Some lines will not be
// properly parsed.
let regex_tree = RegexTree::root(r"(?P<nums>[\d\s]*) (?P<yn>true|false)")
    .with_child("nums", RegexTree::leaf(r"\d+"))
    .build();

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Line {
    nums: Vec<i32>,
    yn: bool,
}

let uncaptured: Vec<&str> = get_uncaptured(&regex_tree, &text).collect();

// We could inspect the output and notice the numbers, and negative sign, which
// we had wanted to capture but did not
assert_eq!(vec!["\n8 3 -", "\n"], uncaptured);
```

## Example use case
The following data file is being maintained by hand, but we want it in a
more structured format. We need to extract names and a list of the favorite
numbers associated with those names.
```text
Lina's favorite number is 2
Selah's favorite numbers are 3, 6, 8, and 12
Aili's favorite numbers are 1, 4, 10, and 13
Gemma's favorite numbers are 9, 10, 11, and 14
Genoveva's favorite numbers are 2, 10, 11, 13, and 15
Eryk's favorite number is 6
Alpheus's favorite numbers are 1, 6, 12, and 19
Sven's favorite numbers are 1 and 5
Annabella's favorite numbers are 2, 6, and 14
```

To parse each line, we design the following regular expression
```regex
(?P<name>.*)'s favorite numbers? (?:is|are) (?P<favorite_nums>.*)
```
and call it `line`. After running this on the file, we have a list of
matches. In each, the first capture group is the name, as desired. The
second capture group looks like this:
```text
2
12
13
9, 10, 11, and 14
2, 10, 11, 13, and 15
6
1, 6, 12, and 19
1 and 5
2, 6, and 14
```

To parse the numbers, we design the regular expression
```regex
\d+
```
and call it `favorite_nums`. After running this on each of the second capture
groups from before, capture group zero will contain just one number. We
replace each of the second capture group entries with a list of numbers.

We now have structured data extracted from the file. In a JSON
representation and with some additional metadata, it looks like this:
```json
[
    {
        "name": "Lina",
        "favorite_nums": ["2"]
    },
    {
        "name": "Selah",
        "favorite_nums": ["3", "6", "8", "12"]
    },
    {
        "name": "Aili",
        "favorite_nums": ["1", "4", "10", "13"]
    },
    {
        "name": "Gemma",
        "favorite_nums": ["9", "10", "11", "14"]
    },
    {
        "name": "Genoveva",
        "favorite_nums": ["2", "10", "11", "13", "15"]
    },
    {
        "name": "Eryk",
        "favorite_nums": ["6"]
    },
    {
        "name": "Alpheus",
        "favorite_nums": ["1", "6", "12", "19"]
    },
    {
        "name": "Sven",
        "favorite_nums": ["1", "5"]
    },
    {
        "name": "Annabella",
        "favorite_nums": ["2", "6", "14"]
    }
]
```

Corresponding code is available under `tests/favorite_numbers.rs`.

## Features
- `deserialize-regex-tree`: implements `Deserialize` for `RegexTree`. This
allows users to provide a regex tree as a file and easily customize parsing at
runtime.