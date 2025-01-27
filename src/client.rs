use std::path::PathBuf;

use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::USER_AGENT;

use crate::models::RedditComment;
use crate::models::RedditLink;
use crate::models::RedditResponse;
use crate::models::RedditThing;
use crate::rate_limit::rate_limited_fetch;

#[derive(Debug)]
pub struct SubredditSlug(String);
impl AsRef<str> for SubredditSlug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for SubredditSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl SubredditSlug {
    pub fn new(subreddit: impl AsRef<str>) -> Self {
        Self(subreddit.as_ref().to_string())
    }
}

pub async fn fetch_subreddit_posts(subreddit: SubredditSlug) -> eyre::Result<Vec<RedditLink>> {
    let cache_file = PathBuf::from("target/response.json");
    let response_text = match tokio::fs::try_exists(&cache_file).await {
        Ok(true) => tokio::fs::read_to_string(&cache_file).await?,
        _ => {
            let url = format!("https://www.reddit.com/r/{}.json?raw_json=1", subreddit);
            let mut headers = HeaderMap::new();
            headers.insert(
                USER_AGENT,
                HeaderValue::from_str("windows:ca.teamdman.myredditapp:v0.0.1 (by /u/TeamDman)")?,
            );
            let client = reqwest::Client::builder()
                .default_headers(headers)
                .build()?;
            let response = client.get(url).send().await?;
            let response = response.error_for_status()?;
            let response_text = response.text().await?;
            // let response_value: serde_json::Value = serde_json::from_str(&response_text)?;
            // let response_text = serde_json::to_string_pretty(&response_value)?; // pretty print
            tokio::fs::write(&cache_file, &response_text).await?;
            response_text
        }
    };
    let jd = &mut serde_json::Deserializer::from_str(&response_text);
    let response: RedditResponse = serde_path_to_error::deserialize(jd)?;
    let RedditResponse::Listing(listing) = response;
    let links = listing
        .children
        .into_iter()
        .filter_map(|thing| match thing {
            RedditThing::Link(link) => Some(link),
            _ => None,
        })
        .collect();
    Ok(links)
}

use tokio::fs as tokio_fs; // for async file ops

pub async fn fetch_subreddit_posts_paginated(
    subreddit: SubredditSlug,
    pages: usize,
) -> eyre::Result<Vec<RedditLink>> {
    let mut all_links = Vec::new();
    let mut after: Option<String> = None;

    // We only need one client:
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str("windows:ca.teamdman.myredditapp:v0.0.1 (by /u/TeamDman)")?,
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    for page_idx in 0..pages {
        // Build the cache path
        let cache_file = format!("target/cache/subreddit/{}_{}.json", subreddit, page_idx);
        let cache_path = PathBuf::from(&cache_file);

        // Build the URL with after param
        let mut url = format!("https://www.reddit.com/r/{}.json?raw_json=1", subreddit);
        if let Some(ref a) = after {
            url = format!("{}&after={}", url, a);
        }

        // Try cache
        let response_text = if tokio_fs::try_exists(&cache_path).await.unwrap_or(false) {
            // If the file exists, read from the cache
            tokio_fs::read_to_string(&cache_path).await?
        } else {
            // Otherwise do a network fetch, but with our rate limit
            let response_text = rate_limited_fetch(&client, &url).await?;
            // Save to cache
            if let Some(parent) = cache_path.parent() {
                tokio_fs::create_dir_all(parent).await?;
            }
            tokio_fs::write(&cache_path, &response_text).await?;
            response_text
        };

        // Deserialize
        let jd = &mut serde_json::Deserializer::from_str(&response_text);
        let response: RedditResponse = serde_path_to_error::deserialize(jd)?;
        let RedditResponse::Listing(listing) = response;

        // Collect the links
        listing.children.into_iter().for_each(|thing| {
            if let RedditThing::Link(link) = thing {
                all_links.push(link);
            }
        });

        // Update `after`, so next iteration can request the following page
        after = listing.after;

        // If no more pages, break early
        if after.is_none() {
            break;
        }
    }
    Ok(all_links)
}
pub async fn fetch_link_comments(
    client: &reqwest::Client,
    post_id: &str,
    link_url: &str,
) -> eyre::Result<Vec<RedditComment>> {
    // "target/cache/posts/{post_id}.json"
    let cache_file = format!("target/cache/posts/{}.json", post_id);
    let cache_path = PathBuf::from(&cache_file);

    let response_text = if tokio::fs::try_exists(&cache_path).await.unwrap_or(false) {
        tokio::fs::read_to_string(&cache_path).await?
    } else {
        let url = format!("{}.json?raw_json=1", link_url);
        // Rate-limited fetch
        let response_text = rate_limited_fetch(client, &url).await?;

        if let Some(parent) = cache_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&cache_path, &response_text).await?;
        response_text
    };

    let jd = &mut serde_json::Deserializer::from_str(&response_text);
    let response: (RedditResponse, RedditResponse) = serde_path_to_error::deserialize(jd)?;
    let (_sub, comments) = response;
    let RedditResponse::Listing(listing) = comments;
    let comms = listing
        .children
        .into_iter()
        .filter_map(|thing| {
            if let RedditThing::Comment(comment) = thing {
                Some(comment)
            } else {
                None
            }
        })
        .collect();
    Ok(comms)
}

#[cfg(test)]
mod tests {
    use eyre::bail;
    use serde::Deserialize;

    use crate::models::RedditListing;

    use super::*;

    #[tokio::test]
    async fn deserialize() -> eyre::Result<()> {
        let x = r#"
{
  "data": {
    "after": "t3_1i57pyj",
    "before": null,
    "children": [],
    "dist": 27,
    "geo_filter": null,
    "modhash": ""
  },
  "kind": "Listing"
}
        "#;
        let response = serde_json::from_str::<RedditResponse>(x)?;
        assert_eq!(
            response,
            RedditResponse::Listing(RedditListing {
                modhash: "".to_string(),
                dist: Some(27),
                children: vec![],
                after: Some("t3_1i57pyj".to_string()),
                before: None,
            })
        );
        Ok(())
    }

    #[test]
    fn idk() -> eyre::Result<()> {
        let x = "[1,2,3]";
        let response = serde_json::from_str::<(i32, i32, i32)>(x)?;
        assert_eq!(response, (1, 2, 3));
        Ok(())
    }
    #[test]
    fn idk2() -> eyre::Result<()> {
        let x = r#"[
        {
  "data": {
    "after": "t3_1i57pyj",
    "before": null,
    "children": [],
    "dist": 27,
    "geo_filter": null,
    "modhash": ""
  },
  "kind": "Listing"
},
{
  "data": {
    "after": "t3_1i57pyj",
    "before": null,
    "children": [],
    "dist": 27,
    "geo_filter": null,
    "modhash": ""
  },
  "kind": "Listing"
}]"#;
        let response = serde_json::from_str::<(RedditResponse, RedditResponse)>(x)?;
        Ok(())
    }
    #[test]
    fn lines() -> eyre::Result<()> {
        #[derive(Deserialize, Debug)]
        struct Bruh {
            bruh: String,
        }
        let x = r#"[
    {"bruh": "bruh"},
    {"bruh": "bruh"},
    {"bruh": 1.23},
    {"bruh": "bruh"},
    {"bruh": "bruh"}
]"#;
        let response = serde_json::from_str::<Vec<Bruh>>(x);
        let Err(e) = response else {
            bail!("Expected an error, got {:?}", response);
        };
        println!("{}", e);
        assert_eq!(e.to_string(), "expected value at line 2 column 5");
        Ok(())
    }
}
