use std::path::PathBuf;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

use crate::models::{RedditComment, RedditLink, RedditResponse, RedditThing};

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
        Ok(true) => {
            let response_text = tokio::fs::read_to_string(&cache_file).await?;
            response_text
        }
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

    let response = serde_json::from_str::<RedditResponse>(&response_text)?;
    let RedditResponse::Listing(listing) = response;
    let links = listing.children.into_iter().filter_map(|thing| {
        match thing {
            RedditThing::Link(link) => Some(link),
            _ => None,
        }
    }).collect();
    Ok(links)
}


pub async fn fetch_link_comments(link_url: &str) -> eyre::Result<Vec<RedditComment>> {
    let cache_file = PathBuf::from("target/comments.json");
    let response_text = match tokio::fs::try_exists(&cache_file).await {
        Ok(true) => {
            let response_text = tokio::fs::read_to_string(&cache_file).await?;
            response_text
        }
        _ => {
            let url = format!("{}.json?raw_json=1", link_url);
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

    let response = serde_json::from_str::<(RedditResponse, RedditResponse)>(&response_text)?;
    let (_sub, comments) = response;
    let RedditResponse::Listing(comments) = comments;
    let links = comments.children.into_iter().filter_map(|thing| {
        match thing {
            RedditThing::Comment(comment) => Some(comment),
            _ => None,
        }
    }).collect();
    Ok(links)
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
        assert_eq!(response, RedditResponse::Listing(RedditListing {
            modhash: "".to_string(),
            dist: 27,
            children: vec![],
            after: Some("t3_1i57pyj".to_string()),
            before: None,
        }));
        Ok(())
    }

    #[test]
    fn idk() -> eyre::Result<()> {
        let x = "[1,2,3]";
        let response = serde_json::from_str::<(i32,i32,i32)>(x)?;
        assert_eq!(response, (1,2,3));
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
