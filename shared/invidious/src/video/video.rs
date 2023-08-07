use serde::{Deserialize, Serialize};
use crate::fetch::{fetch, FetchError};
use crate::{hidden::*, common::*};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub title: String,
    #[serde(rename = "videoId")]
    pub id: String,
    #[serde(rename = "videoThumbnails")]
    pub thumbnails: Vec<CommonThumbnail>,
    pub storyboards: Vec<Storyboard>,
    pub description: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    pub published: u64,
    #[serde(rename = "publishedText")]
    pub published_text: String,

    pub keywords: Vec<String>,
    #[serde(rename = "viewCount")]
    pub views: u64,
    #[serde(rename = "likeCount")]
    pub likes: u32,
    #[serde(rename = "dislikeCount")]
    pub dislikes: u32,

    pub paid: bool,
    pub premium: bool,
    #[serde(rename = "isFamilyFriendly")]
    pub family_friendly: bool,
    #[serde(rename = "allowedRegions")]
    pub allowed_regions: Vec<CountryCode>,
    pub genre: String,
    #[serde(rename = "genreUrl")]
    pub genre_url: String,

    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorUrl")]
    pub author_url: String,
    #[serde(rename = "authorThumbnails")]
    pub author_thumbnails: Vec<CommonImage>,

    #[serde(rename = "subCountText")]
    pub sub_count_text: String,
    #[serde(rename = "lengthSeconds")]
    pub length: u32,
    #[serde(rename = "allowRatings")]
    pub allow_ratings: bool,
    pub rating: f32,
    #[serde(rename = "isListed")]
    pub listed: bool,
    #[serde(rename = "liveNow")]
    pub live: bool,
    #[serde(rename = "isUpcoming")]
    pub upcoming: bool,
    #[serde(rename = "premiereTimestamp")]
    #[serde(default)]
    pub premiere_timestamp: u64,
    #[serde(rename = "dashUrl")]
    pub dash: String,

    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Vec<AdaptiveFormat>,
    #[serde(rename = "formatStreams")]
    pub format_streams: Vec<FormatStream>,

    pub captions: Vec<Caption>,

    #[serde(rename = "recommendedVideos")]
    pub recommended_videos: Vec<VideoShort>,
}

impl Video {
    fn url(server: &str, id: &str) -> String {
        format!("{server}/api/v1/videos/{id}")
    }

    pub async fn fetch_video(server: &str, id: &str, args: Option<&str>) -> Result<Self, FetchError> {
        let video_url: String = Self::url(server, id);
        let video_json: String = fetch(&video_url).await?;
        let video: Self = serde_json::from_str(&video_json)?;
        Ok(video)
    }

    pub fn video_link(&self, server: &str) -> String {
        format!("{}/latest_version?id={}", server, &self.id)
    }
}

impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        *&self.id.eq(&other.id) &&
            *&self.title.eq(&other.title) &&
            *&self.author_id.eq(&other.author_id)
    }
}