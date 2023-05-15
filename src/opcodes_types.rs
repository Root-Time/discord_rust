use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<T = Option<Value>> {
    pub t: Option<String>,
    pub s: Option<i64>,
    pub op: i64,
    pub d: T,
}

#[derive(Deserialize, Debug)]
pub struct Hello {
    pub _trace: Value,
    pub heartbeat_interval: i64,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<i32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<i64>,
    pub premium_type: Option<i64>,
    pub public_flags: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Application {
    pub flags: i64,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildPreview {
    pub id: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    // pub emojis:
    // pub features:
    pub approximate_member_count: Option<i64>,
    pub approximate_presence_count: Option<i64>,
    pub description: Option<String>,
    // pub stickers
}

#[derive(Deserialize, Debug)]
pub struct Ready {
    pub v: i16,
    pub user: User,
    pub guilds: Vec<GuildPreview>,
    pub session_id: Option<String>,
    pub resume_gateway_url: String,
    // pub shard: Option
    pub application: Application,
}

#[derive(Deserialize, Debug)]
pub struct VoiceStateUpdate {
    #[serde(rename = "channel_id")]
    pub channel_id: Option<String>,
    pub deaf: bool,
    #[serde(rename = "guild_id")]
    pub guild_id: String,
    // pub member: GuildMember,
    pub mute: bool,
    #[serde(rename = "request_to_speak_timestamp")]
    pub request_to_speak_timestamp: Value,
    #[serde(rename = "self_deaf")]
    pub self_deaf: bool,
    #[serde(rename = "self_mute")]
    pub self_mute: bool,
    #[serde(rename = "self_video")]
    pub self_video: bool,
    #[serde(rename = "session_id")]
    pub session_id: String,
    pub suppress: bool,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

#[derive(Deserialize, Debug)]
pub struct GuildMember {
    pub avatar: Value,
    #[serde(rename = "communication_disabled_until")]
    pub communication_disabled_until: Value,
    pub deaf: bool,
    pub flags: i64,
    #[serde(rename = "joined_at")]
    pub joined_at: String,
    pub mute: bool,
    pub nick: Value,
    pub pending: bool,
    #[serde(rename = "premium_since")]
    pub premium_since: Value,
    pub roles: Vec<Value>,
    pub user: User,
    pub permissions: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildCreate {}
// pub struct HeartbeatRefreshed {

// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct D {
//     #[serde(rename = "_trace")]
//     pub trace: Vec<(String, Trace)>,
//     pub application: Application,
//     #[serde(rename = "geo_ordered_rtc_regions")]
//     pub geo_ordered_rtc_regions: Vec<String>,
//     #[serde(rename = "guild_join_requests")]
//     pub guild_join_requests: Vec<Value>,
//     pub guilds: Vec<Guild>,
//     pub presences: Vec<Value>,
//     #[serde(rename = "private_channels")]
//     pub private_channels: Vec<Value>,
//     pub relationships: Vec<Value>,
//     #[serde(rename = "resume_gateway_url")]
//     pub resume_gateway_url: String,
//     #[serde(rename = "session_id")]
//     pub session_id: String,
//     #[serde(rename = "session_type")]
//     pub session_type: String,
//     pub user: User,
//     #[serde(rename = "user_settings")]
//     pub user_settings: UserSettings,
//     pub v: i64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Trace {
//     pub micros: i64,
//     pub calls: (
//         String,
//         Calls,
//         String,
//         Calls2,
//         String,
//         Calls3,
//         String,
//         Calls4,
//     ),
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Calls {
//     pub micros: i64,
//     pub calls: Vec<Value>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Calls2 {
//     pub micros: i64,
//     pub calls: Vec<Value>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Calls3 {
//     pub micros: i64,
//     pub calls: Vec<Value>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Calls4 {
//     pub micros: i64,
//     pub calls: Vec<Value>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Application {
//     pub flags: i64,
//     pub id: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Guild {
//     pub id: String,
//     pub unavailable: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct User {
//     pub avatar: Value,
//     pub bot: bool,
//     pub discriminator: String,
//     #[serde(rename = "display_name")]
//     pub display_name: Value,
//     pub email: Value,
//     pub flags: i64,
//     #[serde(rename = "global_name")]
//     pub global_name: Value,
//     pub id: String,
//     #[serde(rename = "mfa_enabled")]
//     pub mfa_enabled: bool,
//     #[serde(rename = "public_flags")]
//     pub public_flags: i64,
//     pub username: String,
//     pub verified: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct UserSettings {}
