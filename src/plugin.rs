use async_trait::async_trait;
use std::error::Error;

use crate::{
    downloader::Downloader,
    query::{QueryInfo, QueryResult},
    streamer::Streamer,
};

/// Defines the behaviour of any valid plugin
#[async_trait]
pub trait Plugin {
    /// Returns the HTTP method that must
    /// be used to build up requests
    fn method(&self) -> reqwest::Method;

    /// Returns the url to which the plugin
    /// sends its requests
    fn base_url(&self) -> &'static str;

    /// Creates a requst for the plugin service
    fn query(
        &self,
        info: &QueryInfo,
        req_builder: reqwest::RequestBuilder,
    ) -> Result<reqwest::Request, Box<dyn std::error::Error>>;

    /// Parses the result of a request created with the `query` method and
    /// extracts the required data
    async fn parse(
        &self,
        info: &QueryInfo,
        resp: reqwest::Response,
    ) -> Result<QueryResult, Box<dyn Error>>;

    /// Returns a `Downloader` instance that
    /// can be used to download media from the
    /// plugin service
    fn download(&self) -> Downloader;

    /// Returns a `Streamer` instance that
    /// can be used to stream media from the
    /// plugin service
    fn stream(&self) -> Streamer;
}
