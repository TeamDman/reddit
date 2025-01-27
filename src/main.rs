use client::fetch_link_comments;
use client::fetch_subreddit_posts;
use client::SubredditSlug;

pub mod client;
pub mod models;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let sub = SubredditSlug::new("bapcsalescanada");
    let links = fetch_subreddit_posts(sub).await?;
    for link in &links {
        println!("Title: {}", link.title);
    }

    let link = links.first().ok_or(eyre::eyre!("No links found"))?;
    println!("First link: {}", link.title);
    let url = link.url.as_str();
    println!("First link URL: {}", url);
    let comments = fetch_link_comments(&link.url).await?;
    for comment in &comments {
        println!(
            "Comment ({}{}): {}",
            comment.author,
            comment
                .author_flair_text
                .as_ref()
                .map(|x| format!(" - {}", x))
                .unwrap_or_default(),
            comment.body
        );
    }
    Ok(())
}
