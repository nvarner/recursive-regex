use itertools::Itertools;
use regex::Captures;
use std::iter;

pub fn get_uncaptured_by_one<'t, 'c>(
    text: &'t str,
    captures: &'c Captures<'t>,
) -> impl Iterator<Item = &'t str> + 'c {
    let before = iter::once((0, 0));
    let between = captures
        .iter()
        .flatten()
        .map(|cap| (cap.start(), cap.end()));
    let after = iter::once((text.len(), text.len()));
    let all = before.chain(between).chain(after);

    all.tuple_windows()
        .map(|((_, end), (start, _))| end..start)
        .filter(|range| !range.is_empty())
        .map(|range| &text[range])
}
