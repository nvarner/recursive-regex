use regex::{CaptureLocations, Captures};

/// Analagous to [`regex::SubCapturesPosIter`], except that the
/// [`regex::CaptureLocations`]
pub struct OwningCaptureLocationsIter {
    idx: usize,
    locs: CaptureLocations,
}

pub struct OwningCapturesIter<'t> {
    caps: Captures<'t>,
    locs: CaptureLocations,
}
