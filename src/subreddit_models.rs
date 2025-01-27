use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag="kind", content="data")]
pub enum RedditResponse {
    Listing(RedditListing),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RedditListing {
    pub modhash: String,
    pub dist: i64,
    pub children: Vec<RedditThing>,
    pub after: Option<String>,
    pub before: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag="kind", content="data")]
pub enum RedditThing {
    #[serde(rename = "t1")]
    Comment,
    #[serde(rename = "t2")]
    Account,
    #[serde(rename = "t3")]
    Link(RedditLink),
    #[serde(rename = "t4")]
    Message,
    #[serde(rename = "t5")]
    Subreddit,
    #[serde(rename = "t6")]
    Award,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RedditLink {
    pub approved_at_utc: Option<serde_json::Value>,
    pub subreddit: Subreddit,
    pub selftext: String,
    pub author_fullname: String,
    pub saved: bool,
    pub mod_reason_title: Option<serde_json::Value>,
    pub gilded: i64,
    pub clicked: bool,
    pub title: String,
    pub link_flair_richtext: Vec<FlairRichtext>,
    pub subreddit_name_prefixed: SubredditNamePrefixed,
    pub hidden: bool,
    pub pwls: i64,
    pub link_flair_css_class: Option<LinkFlairCssClass>,
    pub downs: i64,
    pub thumbnail_height: Option<i64>,
    pub top_awarded_type: Option<serde_json::Value>,
    pub hide_score: bool,
    pub name: String,
    pub quarantine: bool,
    pub link_flair_text_color: Option<FlairTextColor>,
    pub upvote_ratio: f64,
    pub author_flair_background_color: Option<String>,
    pub subreddit_type: SubredditType,
    pub ups: i64,
    pub total_awards_received: i64,
    pub media_embed: Gildings,
    pub thumbnail_width: Option<i64>,
    pub author_flair_template_id: Option<String>,
    pub is_original_content: bool,
    pub user_reports: Vec<Option<serde_json::Value>>,
    pub secure_media: Option<serde_json::Value>,
    pub is_reddit_media_domain: bool,
    pub is_meta: bool,
    pub category: Option<serde_json::Value>,
    pub secure_media_embed: Gildings,
    pub link_flair_text: Option<String>,
    pub can_mod_post: bool,
    pub score: i64,
    pub approved_by: Option<serde_json::Value>,
    pub is_created_from_ads_ui: bool,
    pub author_premium: bool,
    pub thumbnail: String,
    pub edited: bool,
    pub author_flair_css_class: Option<serde_json::Value>,
    pub author_flair_richtext: Vec<FlairRichtext>,
    pub gildings: Gildings,
    pub content_categories: Option<serde_json::Value>,
    pub is_self: bool,
    pub mod_note: Option<serde_json::Value>,
    pub created: f64,
    pub link_flair_type: AuthorFlairType,
    pub wls: i64,
    pub removed_by_category: Option<serde_json::Value>,
    pub banned_by: Option<serde_json::Value>,
    pub author_flair_type: AuthorFlairType,
    pub domain: String,
    pub allow_live_comments: bool,
    pub selftext_html: Option<String>,
    pub likes: Option<serde_json::Value>,
    pub suggested_sort: Option<String>,
    pub banned_at_utc: Option<serde_json::Value>,
    pub view_count: Option<serde_json::Value>,
    pub archived: bool,
    pub no_follow: bool,
    pub is_crosspostable: bool,
    pub pinned: bool,
    pub over_18: bool,
    pub all_awardings: Vec<Option<serde_json::Value>>,
    pub awarders: Vec<Option<serde_json::Value>>,
    pub media_only: bool,
    pub link_flair_template_id: Option<String>,
    pub can_gild: bool,
    pub spoiler: bool,
    pub locked: bool,
    pub author_flair_text: Option<String>,
    pub treatment_tags: Vec<Option<serde_json::Value>>,
    pub visited: bool,
    pub removed_by: Option<serde_json::Value>,
    pub num_reports: Option<serde_json::Value>,
    pub distinguished: Option<serde_json::Value>,
    pub subreddit_id: SubredditId,
    pub author_is_blocked: bool,
    pub mod_reason_by: Option<serde_json::Value>,
    pub removal_reason: Option<serde_json::Value>,
    pub link_flair_background_color: LinkFlairBackgroundColor,
    pub id: String,
    pub is_robot_indexable: bool,
    pub report_reasons: Option<serde_json::Value>,
    pub author: String,
    pub discussion_type: Option<serde_json::Value>,
    pub num_comments: i64,
    pub send_replies: bool,
    pub contest_mode: bool,
    pub mod_reports: Vec<Option<serde_json::Value>>,
    pub author_patreon_flair: bool,
    pub author_flair_text_color: Option<FlairTextColor>,
    pub permalink: String,
    pub stickied: bool,
    pub url: String,
    pub subreddit_subscribers: i64,
    pub created_utc: f64,
    pub num_crossposts: i64,
    pub media: Option<serde_json::Value>,
    pub is_video: bool,
    pub author_cakeday: Option<bool>,
    pub post_hint: Option<String>,
    pub preview: Option<Preview>,
    pub url_overridden_by_dest: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct FlairRichtext {
    pub e: AuthorFlairType,
    pub t: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuthorFlairType {
    Richtext,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FlairTextColor {
    Dark,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Gildings {
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum LinkFlairBackgroundColor {
    #[serde(rename = "#dadada")]
    Dadada,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "#fbe9d0")]
    Fbe9D0,
    #[serde(rename = "#187718")]
    The187718,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum LinkFlairCssClass {
    Comment,
    #[serde(rename = "expired")]
    Expired,
    Review,
    #[serde(rename = "WeeklyDiscussion")]
    WeeklyDiscussion,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Preview {
    pub images: Vec<Image>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Image {
    pub source: Source,
    pub resolutions: Vec<Source>,
    pub variants: Gildings,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Source {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Subreddit {
    Bapcsalescanada,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubredditId {
    #[serde(rename = "t5_2tesr")]
    T52Tesr,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum SubredditNamePrefixed {
    #[serde(rename = "r/bapcsalescanada")]
    RBapcsalescanada,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubredditType {
    Public,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    T3,
}
