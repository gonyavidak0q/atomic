pub mod api {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct UnlockRequest {
        pub username: String,
        pub password: String,
        pub items: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UnlockResponse {
        pub success: bool,
        pub message: String,
    }

    pub async fn unlock_items(client: &Client, request: UnlockRequest) -> Result<UnlockResponse, reqwest::Error> {
        let response = client.post("https://api.fortnite.com/unlock")
            .json(&request)
            .send()
            .await?;
        response.json().await
    }
}