// main.rs

use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://www.example.com";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response body:\n{}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

