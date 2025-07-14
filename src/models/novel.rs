use std::collections::HashMap;

use super::{user::User, *};
use serde_json::Value;
use serde_with::{serde_as, DefaultOnError};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Response {
    pub novels: Vec<Novel>,
    pub next_url: Option<String>,
}
crate::impl_next_url!(Response);

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Novel {
    pub id: i64,
    pub title: String,
    pub caption: String,
    pub restrict: i32,
    pub x_restrict: i32,
    pub image_urls: ImageUrls,
    pub create_date: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub page_count: i32,
    pub text_length: i32,
    pub user: User,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub series: Option<Series>, // `series` can be {} (empty object)
    pub is_bookmarked: bool,
    pub total_bookmarks: i32,
    pub total_view: i32,
    pub visible: bool,
    pub total_comments: i32,
    pub is_muted: bool,
    pub is_mypixiv_only: bool,
    pub is_x_restricted: bool,
    pub is_original: Option<bool>,
    pub novel_ai_type: Option<i64>,
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct NovelTextResponse {
    // novel_marker: NovelMarker,
    pub novel_text: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub series_prev: Option<Novel>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub series_next: Option<Novel>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebViewNovelResponse {
    title: String,
    user_id: String,
    text: String,
    tags: Vec<String>,
    series_title: Option<String>,
    series_id: Option<String>,
    rating: Rating,
    is_original: bool,
    images: ArrayOrMap<Image>,
    id: String,
    cover_url: String,
    cdate: String,
    caption: String,
    ai_type: i64,
    series_is_watched: Option<bool>,
    series_navigation: Option<SeriesNavigation>,
    marker: Option<Value>,
    seasonal_effect_animation_urls: Option<Value>,
    replaceable_item_ids: Option<Vec<Value>>,
    glossary_items: Vec<Value>,
    illusts: ArrayOrMap<Value>,
    event_banners: Option<Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum ArrayOrMap<V> {
    Array(Vec<V>),
    Map(HashMap<String, V>),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    novel_image_id: String,
    sl: String,
    urls: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Rating {
    bookmark: u64,
    like: u64,
    view: u64,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SeriesNavigation {
    next_novel: Option<NovelPrevNext>,
    prev_novel: Option<NovelPrevNext>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NovelPrevNext {
    content_order: String,
    cover_url: String,
    id: u64,
    title: String,
    viewable: bool,
    viewable_message: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct NovelSeriesDetail1 {
    caption: String,
    content_count: i64,
    display_text: String,
    id: i64,
    is_concluded: bool,
    is_original: bool,
    novel_ai_type: i64,
    title: String,
    total_character_count: i64,
    user: User,
    watchlist_added: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SeriesDetail {
    next_url: Option<String>,
    novel_series_detail: NovelSeriesDetail1,
    novel_series_first_novel: Novel,
    novel_series_latest_novel: Novel,
    novels: Vec<Novel>,
}
