use std::collections::HashMap;

use reqwest::header::AUTHORIZATION;

use crate::opcodes_types::{User, VoiceStateUpdate};

pub async fn on_voice_channel_create(state: VoiceStateUpdate) {
    // println!("{:?}", state.channel_id.unwrap_or_default())

    let user = get_user(state.user_id).await;

    let data = CreateChannelData {
        guild_id: state.guild_id,
        name: "Channel of ".to_string() + user.username.as_str(),
        parent_id: "1066465107665748129".to_string(),
    };

    create_voice_channel(data).await
}

async fn get_user(user_id: String) -> User {
    let url = format!("{}/users/{}", base_api_url(), user_id);

    let user: User = serde_json::from_str(&get_discord_api(url).await).unwrap();

    user
}

fn base_api_url() -> &'static str {
    return "https://discord.com/api/v10";
}

async fn get_discord_api(url: String) -> String {
    let client = reqwest::Client::new();

    let result = client
        .get(url)
        .header(
            AUTHORIZATION,
            "Bot MTA2NjQ2NDcyMzY4OTgxNjIxNA.G-HqYU.vHUxiZYL3nFd0hktCX2WDJD3qLEDFDGKaa8pmc",
        )
        .send()
        .await
        .unwrap();

    return result.text().await.unwrap();
}

struct CreateChannelData {
    guild_id: String,
    parent_id: String,
    name: String,
    // permission_overwrites
}

async fn create_voice_channel(data: CreateChannelData) {
    let guild_id = data.guild_id;
    let url = format!("https://discord.com/api/v10/guilds/{}/channels", guild_id);
    // let url = format!("https://discord.com/api/v10/channels/1066465107665748132");

    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("name", data.name.as_str());
    map.insert("parent_id", data.parent_id.as_str());
    map.insert("type", "2");

    // let data = format!(
    //     r#"{{
    //         'name': 'Test',
    //         'type': 2,
    // 	}}"#
    // );

    // map.insert("parent_id", data.parent_id);

    // map.insert("body", "json");

    let result = client
        .post(url)
        .header(
            AUTHORIZATION,
            "Bot MTA2NjQ2NDcyMzY4OTgxNjIxNA.G-HqYU.vHUxiZYL3nFd0hktCX2WDJD3qLEDFDGKaa8pmc",
        )
        .json(&map)
        .send()
        .await
        .unwrap();

    println!("{:?}", result.text().await.unwrap())
}
