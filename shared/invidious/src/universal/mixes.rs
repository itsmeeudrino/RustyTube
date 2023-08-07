use serde::{Deserialize, Serialize};
use crate::fetch::{fetch, FetchError};
use crate::hidden::MixVideo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mix {
    pub title: String,
    #[serde(rename = "midId")]
    pub id: String,
    pub videos: Vec<MixVideo>,
}

impl Mix {
    fn url(server: &str, args: &str) -> String {
        format!("{}/api/v1/mixes/{}", server, args)
    }

    async fn fetch_mix(server: &str, args: &str) -> Result<Self, FetchError> {
        let mix_url: String = Self::url(server, args);
        let mix_json: String = fetch(&mix_url).await?;
        let mix: Self = serde_json::from_str(&mix_json)?;
        Ok(mix)
    }
}