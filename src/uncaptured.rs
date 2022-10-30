use itertools::Itertools;
use regex::Matches;
use std::iter;

pub fn get_uncaptured<'r, 't: 'r>(
    text: &'t str,
    matches: Matches<'r, 't>,
) -> impl Iterator<Item = &'t str> + 'r {
    let before = iter::once((0, 0));
    let between = matches.map(|re_match| (re_match.start(), re_match.end()));
    let after = iter::once((text.len(), text.len()));
    let all = before.chain(between).chain(after);

    all.tuple_windows()
        .map(|((_, end), (start, _))| end..start)
        .filter(|range| !range.is_empty())
        .map(|range| &text[range])
}
