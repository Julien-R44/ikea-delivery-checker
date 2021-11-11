use reqwest::Response;
use serde::Serialize;
#[derive(Serialize, Clone)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<i32>,
}

#[derive(Serialize)]
struct DiscordMessage {
    username: Option<String>,
    avatar_url: Option<String>,
    content: Option<String>,
    tts: Option<bool>,
    embeds: Vec<Embed>
}

pub async fn send_discord_message(
    message: &str,
    color: i32,
    url: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    return Ok(reqwest::Client::new()
        .post(url)
        .json(&DiscordMessage {
            username: None,
            avatar_url: None,
            content: None,
            tts: Some(false),
            embeds: vec![Embed {
                title: Some("ikea-delivery-checker".to_string()),
                description: Some(message.to_string()),
                url: None,
                timestamp: None,
                color: Some(color),
            }],
        })
        .send()
        .await?);
}
