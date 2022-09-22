use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::Child;
use std::sync::mpsc::{Receiver, Sender};

/// The prototype of functions that
/// are used to initialize and update
/// progress bars associated to downloads
pub type ProgressFollowerFn = fn(Receiver<f32>);

/// The prototype of functions accepted by
/// `Downloaders`s instances
type DownloadFn = fn(&str, &str) -> Result<Child, Box<dyn Error>>;
pub struct Downloader {
    download_fn: DownloadFn,
}

/// A structure that allows
/// the creation of a `Downloader`
pub struct DownloadBuilder;
impl DownloadBuilder {
    pub fn new(download_fn: DownloadFn) -> Downloader {
        Downloader { download_fn }
    }
}

impl Downloader {
    /// Used the provided `download_fn` to
    /// perform the actual download. Download progress
    /// information are sent through `tx` channel as
    pub fn download(
        &self,
        url: &str,
        file_name: &str,
        tx: Sender<f32>,
    ) -> Result<String, Box<dyn Error>> {
        let mut handler = (self.download_fn)(url, file_name)?;
        let mut out = BufReader::new(handler.stdout.as_mut().unwrap());

        let regex = regex::Regex::new(r"([0-9]{1,3}\.[0-9]{0,1})%$")?;
        loop {
            let mut vec = vec![];
            let _bytes_read = out.read_until(b'%', &mut vec);

            let result = regex.captures(std::str::from_utf8(&vec).unwrap());
            match result {
                Some(result) => {
                    if let Some(percentage) = result.get(1) {
                        let value: f32 = percentage.as_str().parse().unwrap();
                        tx.send(value)?;
                        if value == 100.0 {
                            break;
                        }
                    }
                }
                None => (),
            }
        }

        handler.wait()?;

        Ok(file_name.to_owned() + ".mp3")
    }
}
