use std::{error::Error, time::Duration};

/// Holds the information required
/// to make a query.
pub struct QueryInfo {
    track_name: Option<String>,
    artist_name: Option<String>,
    other_info: Option<String>,
    raw: String,
}

impl QueryInfo {
    /// Construct a `QueryInfo` setting `track_name`, `artist_name` and `other_info` to
    /// the appropriate `Option` variant. If the provided value for an argument is an
    /// empty string (`""`), the selected variant is `None`.
    ///
    /// The `raw` field is set to `"{track_name} {artist_name} {other_info}"`
    pub fn as_detailed(track_name: &str, artist_name: &str, other_info: &str) -> Self {
        let raw = format!("{} {} {}", track_name, artist_name, other_info)
            .trim()
            .to_string();

        let track_name = if track_name == "" {
            None
        } else {
            Some(track_name.to_string())
        };

        let artist_name = if artist_name == "" {
            None
        } else {
            Some(artist_name.to_string())
        };

        let other_info = if other_info == "" {
            None
        } else {
            Some(other_info.to_string())
        };

        QueryInfo {
            track_name,
            artist_name,
            other_info,
            raw,
        }
    }

    /// Construct a `QueryInfo` setting al optional field to `None`
    pub fn as_raw(raw: &str) -> Self {
        QueryInfo {
            track_name: None,
            artist_name: None,
            other_info: None,
            raw: raw.into(),
        }
    }

    pub fn track_name(&self) -> &str {
        if let Some(track_name) = &self.track_name {
            return &track_name;
        }

        ""
    }

    pub fn artist_name(&self) -> &str {
        if let Some(artist_name) = &self.artist_name {
            return &artist_name;
        }

        ""
    }

    pub fn other_info(&self) -> &str {
        if let Some(other_info) = &self.other_info {
            return &other_info;
        }

        ""
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
}

/// Holds the information pulled from the
/// plugin service
#[derive(PartialEq, Clone, PartialOrd, Debug)]
pub struct QueryResult {
    data: Vec<QueryResultData>,
}

impl QueryResult {
    pub fn new(data: Vec<QueryResultData>) -> Self {
        QueryResult { data }
    }

    pub fn data(&self) -> &Vec<QueryResultData> {
        &self.data
    }
}

/// Each plugin has to implement this trait to define how a piece
/// of information pulled from the plugin's service can be
/// deserialized and transformed into the selected result type.
/// - `S`: source data structure
/// - `R`: result data structure
/// - `P`: plugin for which the trait is being implemented
///
/// `P` is necessary to allow implementing this trait when both `S` and
/// `R` are externally defined types
pub trait Deserializable<S, R, P> {
    fn parse(source: &S) -> Result<R, Box<dyn Error>>;
}

#[derive(PartialEq, Clone, PartialOrd, Debug)]
/// Holds the relevant information pulled from
/// plugins' services.
pub struct QueryResultData {
    track_id: String,
    track_name: String,
    track_url: reqwest::Url,
    track_thumbnail: reqwest::Url,
    artist_name: String,
    artist_thumbnail: reqwest::Url,
    duration: Duration,
}

impl QueryResultData {
    pub fn new(
        track_id: &str,
        track_name: &str,
        track_url: reqwest::Url,
        track_thumbnail: reqwest::Url,
        artist_name: &str,
        artist_thumbnail: reqwest::Url,
        duration: Duration,
    ) -> Self {
        QueryResultData {
            track_id: track_id.into(),
            track_name: track_name.into(),
            track_url,
            track_thumbnail,
            artist_name: artist_name.into(),
            artist_thumbnail,
            duration,
        }
    }

    pub fn track_id(&self) -> &str {
        &self.track_id
    }

    pub fn track_name(&self) -> &str {
        &self.track_name
    }

    pub fn track_url(&self) -> &reqwest::Url {
        &self.track_url
    }

    pub fn track_thumbnail(&self) -> &reqwest::Url {
        &self.track_thumbnail
    }

    pub fn artist_name(&self) -> &str {
        &self.artist_name
    }

    pub fn artist_thumbnail(&self) -> &reqwest::Url {
        &self.artist_thumbnail
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }

    pub fn duration_str(duration: std::time::Duration) -> String {
        let secs = duration.as_secs();
        let mut mins: u64 = secs / 60;
        if mins > 60 {
            let hours: u64 = mins / 60;
            mins = mins - hours * 60;
            return format!("{}:{:02}:{:02}", hours, mins, secs - hours * 3600 - mins * 60);
        }
        return format!("{}:{}", mins, secs - mins * 60);
    }
}
