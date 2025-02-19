use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize)]
struct UnlockRequest {
    username: String,
    password: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

async fn unlock_items(client: &Client, request: UnlockRequest) -> Result<UnlockResponse, reqwest::Error> {
    let response = client.post("https://api.fortnite.com/unlock")
        .json(&request)
        .send()
        .await?;
    response.json().await
}

fn main() {
    let client = Client::new();
    let mut username = String::new();
    let mut password = String::new();
    let mut items = String::new();

    println!("Enter your Fortnite username:");
    io::stdin().read_line(&mut username).unwrap();
    println!("Enter your Fortnite password:");
    io::stdin().read_line(&mut password).unwrap();
    println!("Enter the items to unlock (comma separated):");
    io::stdin().read_line(&mut items).unwrap();

    let items: Vec<String> = items.trim().split(',').map(|s| s.trim().to_string()).collect();
    let request = UnlockRequest {
        username: username.trim().to_string(),
        password: password.trim().to_string(),
        items,
    };

    let unlock_future = unlock_items(&client, request);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    match runtime.block_on(unlock_future) {
        Ok(response) => {
            if response.success {
                println!("Items unlocked successfully: {}", response.message);
            } else {
                println!("Failed to unlock items: {}", response.message);
            }
        }
        Err(e) => {
            println!("Error occurred: {}", e);
        }
    }
}