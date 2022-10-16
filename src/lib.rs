//! # Recursive Regex
//!
//! The recursive regex algorithm is designed to be a simple, customizable
//! parser for basic data files. It matches a regular expression to text as many
//! times as possible, extracting data via capture groups. On each capture
//! group, it may recurse with a new regular expression to further parse the
//! results.

#[cfg(test)]
mod tests {
    use super::*;
}
