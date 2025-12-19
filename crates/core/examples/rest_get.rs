use reqoxidize_core::{Client, Request};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    login: String,
    id: u64,
    public_repos: u32,
}

#[tokio::main]
async fn main() -> reqoxidize_core::Result<()> {
    let client = Client::new();

    // GET simple
    let request = Request::get("https://api.github.com/users/octocat")
        .header("User-Agent", "reqoxidize")
        .header("Accept", "application/json")
        .build()?;

    let response = client.execute(request).await?;

    println!("Status: {}", response.status);
    println!("Elapsed: {}ms", response.elapsed_ms);

    if response.is_success() {
        let user: User = response.json()?;
        println!("User: {} (ID: {})", user.login, user.id);
        println!("Repos: {}", user.public_repos);
    }

    // GET con query params
    let request = Request::get("https://api.github.com/search/repositories")
        .header("User-Agent", "reqoxidize")
        .query("q", "rust")
        .query("sort", "stars")
        .query("per_page", "5")
        .build()?;

    let response = client.execute(request).await?;
    println!("\nSearch status: {}", response.status);

    Ok(())
}
