use reqwest::{header::{CONTENT_TYPE, AUTHORIZATION}, Client};
use serde::Deserialize;
use serde_json::from_str;
use anyhow::{Context, Result};

#[derive(Debug, Deserialize)]
struct ResponseData {
    user: User,
}

#[derive(Debug, Deserialize)]
struct User {
    email: String,
    FullName: String,
    pin: String, // Use String instead of u128 as JSON represents it as a string
    wallet_balance: String, // Use String if the JSON represents it as a string
    username: String,
}

async fn maskawa() -> Result<()> {
    let url = "https://www.maskawasub.com/api/user/";
    let client = Client::new();
    let apikey = "Token f9a6955c2465f3bc8265b8ce7ba454969c7f8bdb";
    
    let res = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, apikey)
        .send()
        .await
        .context("Failed to send request")?;

    let body = res.text().await.context("Failed to fetch response body")?;
    let response: ResponseData = from_str(&body)
        .context("Failed to parse JSON")?;
     let user_details = response.user.wallet_balance;
     println!("the user full name is : {} and wallet amount is: {}",response.user.FullName,user_details);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    maskawa().await
}
