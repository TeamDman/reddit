use subreddit_client::{fetch_subreddit_posts, SubredditSlug};
use subreddit_models::{RedditResponse, RedditThing};

pub mod subreddit_models;
pub mod subreddit_client;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let sub = SubredditSlug::new("bapcsalescanada");
    let resp = fetch_subreddit_posts(sub).await?;
    let RedditResponse::Listing(listing) = resp;
    for child in listing.children {
        let RedditThing::Link(link) = child else {
            continue;
        };
        println!("Title: {}", link.title);
    }
    Ok(())
}
