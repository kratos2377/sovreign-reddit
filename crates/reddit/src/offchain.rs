
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sov_modules_macros::offchain;
use std::{sync::{Arc, Mutex, OnceLock}, time::Duration};



static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

#[derive(Serialize , Deserialize)]
pub enum RedditCollections {
    USER,
    SUBREDDIT,
    POST,
}


impl RedditCollections {


    fn to_string(&self) -> String {
        match self {
            RedditCollections::USER => "USER".to_string(),
            RedditCollections::SUBREDDIT => "SUBREDDIT".to_string(),
            RedditCollections::POST => "POST".to_string(),
        }
    }

    // Convert string to enum
    fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "USER" => Ok(RedditCollections::USER),
            "SUBREDDIT" => Ok(RedditCollections::SUBREDDIT),
            "POST" => Ok(RedditCollections::POST),
            _ => Err(format!("Unknown Collection: {}", s)),
        }
    }

}

#[derive(Serialize , Deserialize)]
pub enum ChangeType {
    CREATED,
    UPDATED
}


impl ChangeType {


    fn to_string(&self) -> String {
        match self {
            ChangeType::CREATED => "CREATED".to_string(),
            ChangeType::UPDATED => "UPDATED".to_string(),
        }
    }

    // Convert string to enum
    fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "CREATED" => Ok(ChangeType::CREATED),
            "UPDATED" => Ok(ChangeType::UPDATED),
            _ => Err(format!("Unknown Changetype: {}", s)),
        }
    }

}



#[derive(Serialize , Deserialize)]
pub struct RedditStateChanges {
    pub state: RedditCollections,
    pub change: String,
    pub address: String,
    pub change_type: ChangeType

}

pub fn get_global_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("MyApp/1.0")
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .build()
            .expect("Failed to create HTTP client")
    })
}


#[offchain]
pub async fn publish_state(
    body: RedditStateChanges
)  {


    tokio::spawn( async move {
        let payload_string = serde_json::to_string_pretty(&body).unwrap();
            let _ = get_global_client().post("random_url").body(payload_string).send().await;
    });



}