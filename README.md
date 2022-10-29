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

// Text we want to deseralize, possibly from a file
let text = "1 2 456";

// Construct a regex tree with no children -- more or less just a regular regex
// See `RegexTree::root` for more complex functionality
let regex_tree = RegexTree::leaf(r"\d+");
let deserialized: Vec<u32> = from_regex_tree_and_str(&regex_tree, &text).unwrap();
assert_eq!(deserialized, vec![1, 2, 456]);
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