use std::error::Error;
use std::process::Child;

/// The prototype of functions accepted by
/// `Downloaders`s instances
type StreamFn = fn(&str, &str) -> Result<Child, Box<dyn Error>>;

pub struct Streamer {
    stream_fn: StreamFn,
}

impl Streamer {
    pub fn stream(&self, url: &str, file_name: &str) -> Result<Child, Box<dyn Error>> {
        let handler = (self.stream_fn)(url, file_name)?;
        Ok(handler)
    }
}

/// A structure that allows
/// the creation of a `Streamer`
pub struct StreamBuilder;
impl StreamBuilder {
    pub fn new(stream_fn: StreamFn) -> Streamer {
        Streamer { stream_fn }
    }
}
