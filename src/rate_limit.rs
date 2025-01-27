use std::sync::Mutex;
use std::time::Duration;
use tokio::time::sleep;

static GLOBAL_RATE_LIMITER: Mutex<()> = Mutex::new(());

pub async fn rate_limited_fetch(client: &reqwest::Client, url: &str) -> eyre::Result<String> {
    {
        // Lock the mutex (ensures only one request at a time can proceed).
        // Once locked, we do the sleep; then we do the actual fetch.
        let _guard = GLOBAL_RATE_LIMITER.lock().unwrap();
        // Sleep for 1 second. If you want a more precise limiter, you can refine this logic,
        // but this is a quick and obvious approach:
        sleep(Duration::from_secs(1)).await;
    }
    
    let response = client.get(url).send().await?;
    let response = response.error_for_status()?;
    let text = response.text().await?;
    Ok(text)
}
