use DiscordMessenger::{DiscordMessage, EmbedBuilder};

pub fn send_discord_message(message: &str, color: i32, url: &str) {
    DiscordMessage::new()
    .add_embed(EmbedBuilder::new()
        .set_title("ikea-delivery-checker".to_string())
        .set_description(message.to_string())
        .set_color(color)
        .build())
    .send(url.to_string());
}