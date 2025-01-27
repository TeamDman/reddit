use client::fetch_link_comments;
use client::fetch_subreddit_posts_paginated;
use client::SubredditSlug;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::USER_AGENT;

pub mod client;
pub mod lazy;
pub mod models;
pub mod rate_limit;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    // 1) Fetch 5 pages of posts
    let sub = SubredditSlug::new("bapcsalescanada");
    let all_links = fetch_subreddit_posts_paginated(sub, 5).await?;
    println!("Fetched {} links total", all_links.len());

    // 2) Build a client to reuse for all comment fetches
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str("windows:ca.teamdman.myredditapp:v0.0.1 (by /u/TeamDman)")?,
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // 3) For each post, fetch comments. In a real-world scenario, you might want
    //    concurrency, but that can clash with the 1req/sec limit. We'll do them
    //    sequentially for simplicity:
    for link in &all_links {
        println!("Fetching comments for post {} - '{}'", link.id, link.title);
        let comments = fetch_link_comments(
            &client,
            &link.id,
            format!("https://www.reddit.com/{}", link.permalink).as_ref(),
        )
        .await?;
        println!("  -> Found {} top-level comments", comments.len());
    }

    Ok(())
}
