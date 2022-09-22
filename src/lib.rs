use std::collections::HashMap;

pub mod downloader;
pub mod error;
pub mod plugin;
pub mod query;
pub mod streamer;
use crate::{downloader::*, error::*, plugin::*, query::*};

/// Manages the registered plugins and provides a unique
/// interface toward them. It holds a list of the registered
/// plugins and allows the designation of one as the default
/// plugins for queries.
pub struct PluginManager {
    client: reqwest::Client,
    plugins: HashMap<String, Box<dyn Plugin>>,
    default: Option<String>,
}

impl PluginManager {
    /// Creates a new `PluginManger` with no plugin
    /// registered. To make a call to `query` a plugin
    /// has to be set as default.
    pub fn new() -> Self {
        PluginManager {
            client: reqwest::Client::new(),
            plugins: HashMap::new(),
            default: None,
        }
    }

    /// Add a new plugin to the list of registered ones with name `name`
    pub fn register_plugin(
        &mut self,
        plugin: Box<dyn Plugin>,
        name: &str,
    ) -> Result<(), PluginError> {
        if self.plugins.contains_key(name) {
            return Err(PluginError::DuplicatedPlugin(name.into()));
        }

        self.plugins.insert(name.into(), plugin);
        Ok(())
    }

    /// Removes the plugin `name` from the list of registered ones
    pub fn unregister_plugin(&mut self, name: &str) -> Result<(), PluginError> {
        if !self.plugins.contains_key(name) {
            return Err(PluginError::UnregisteredPlugin(name.into()));
        }

        self.plugins.remove(name);
        Ok(())
    }

    /// Sets the default plugin. It must have been previously
    /// registered
    pub fn set_default(&mut self, name: &str) -> Result<(), PluginError> {
        if !self.plugins.contains_key(name) {
            return Err(PluginError::UnregisteredPlugin(name.into()));
        }

        self.default = Some(name.into());
        Ok(())
    }

    /// Returns a reference to the default plugin
    pub fn default(&self) -> Result<&Box<dyn Plugin>, PluginError> {
        if self.default.is_none() {
            return Err(PluginError::NoDefaultPlugin);
        }

        Ok(self.plugins.get(self.default.as_ref().unwrap()).unwrap())
    }

    /// Executes the query using the default plugin
    pub async fn query(&self, info: QueryInfo) -> Result<QueryResult, Box<dyn std::error::Error>> {
        if self.default.is_none() {
            return Err(Box::new(PluginError::NoDefaultPlugin));
        }

        let plugin = self.plugins.get(self.default.as_ref().unwrap()).unwrap();

        let method = plugin.method();
        let url = reqwest::Url::parse(plugin.base_url())?;

        let request = self.client.request(method, url);
        let request = plugin.query(&info, request)?;

        let response = self.client.execute(request).await?;
        let result = plugin.parse(&info, response).await?;

        Ok(result)
    }

    /// Requests the default plugin to download the media associated
    /// to `url` and to save it as `file_name`. Also, uses the `progress_follower`
    /// to display download progres.
    pub async fn download(
        &self,
        url: &str,
        file_name: &str,
        progress_follower: ProgressFollowerFn,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let default = self.default.as_ref().unwrap();
        let plugin = self.plugins.get(default).unwrap();
        let downloader = plugin.download();

        let (tx, rx) = std::sync::mpsc::channel();

        let pf = std::thread::spawn(move || progress_follower(rx));
        let file_name = downloader.download(url, file_name, tx)?;
        if let Err(_) = pf.join() {
            return Err(Box::new(PluginError::DownloadError(format!(
                "An error occured while joining progress follower thread",
            ))));
        }

        Ok(file_name)
    }

    pub fn stream(&self, url: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let default = self.default.as_ref().unwrap();
        let plugin = self.plugins.get(default).unwrap();

        let streamer = plugin.stream();
        streamer.stream(url, file_name)?;

        Ok(())
    }
}
