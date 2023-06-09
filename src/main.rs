use std::{fs::File, io::Write, time::Duration};

use dotenv::dotenv;
use futures::{SinkExt, StreamExt};
use opcodes_types::{Event, Hello, Ready, VoiceStateUpdate};
use serde::de::DeserializeOwned;
use serde_json::Value;

// use opcodes_types::{OpCode, D};

use std::env;
use tokio::{sync::mpsc::Sender, time::interval};
// use futures::channel::mpsc::Receiver;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Message},
};
// use tungstenite::{connect, Message};
use url::Url;

mod voice;

mod opcodes_types;

async fn connect() -> Result<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    Error,
> {
    let (socket, _response) = connect_async(Url::parse("wss://gateway.discord.gg/").unwrap())
        .await
        .expect("Can't connect");

    Ok(socket)
}

async fn verify(tx: Sender<Message>) {
    let token = env::var("TOKEN").expect("Token could not be load");

    let identifier = format!(
        r#"{{
    	"op": 2,
    		"d": {{
    			"token": "{token}",
    			"intents": 3243773,
    			"properties": {{
    			"os": "linux",
    			"browser": "my_library",
    			"device": "my_library"
    			}}
    		}}
    	}}"#
    );

    tx.send(Message::Text(identifier)).await.expect("TX");
}

async fn event_generater(message: Message) {
    let text = message.to_string();

    if text == "Authentication failed." {
        println!("Invalid Token!!! Exiting!");
        std::process::exit(101);
    }

    let json: Event = serde_json::from_str(&text).expect("OpCode sruct is wrong!! ");

    match json.op {
        0 => event_manger(json.clone()).await,
        10 => hello(change_d(json.d)).await,
        11 => heartbeat_refreshed(json.clone()).await,
        _ => println!("Undefined OpCode: {:?}", json.op),
    }

    // println!("{:?}", json);
}

fn change_d<T: DeserializeOwned>(d: Option<Value>) -> T {
    let new_d: T = serde_json::from_value(d.unwrap()).expect("COULD NOT CHANGED VALUE INTO T");
    new_d
}

#[allow(dead_code)]
fn change_event_type<T: DeserializeOwned>(opcode: Event) -> Event<T> {
    let new_struct = Event::<T> {
        t: opcode.t,
        op: opcode.op,
        s: opcode.s,
        d: serde_json::from_value(opcode.d.unwrap()).unwrap(),
    };

    new_struct
}

async fn event_manger(text: Event) {
    // println!("T {:?}", text.t.as_deref().unwrap_or_default());

    // return;

    match text.t.as_deref().unwrap_or_default() {
        "READY" => on_ready(change_d(text.d)).await,
        "GUILD_CREATE" => on_guild_create(change_d(text.d)).await,
        "MESSAGE_CREATE" => on_message(change_d(text.d)).await,
        "VOICE_STATE_UPDATE" => on_voice_state_update(change_d(text.d)).await,
        _ => println!("Unkown Event {:?}", text.t.as_deref().unwrap_or_default()),
    };
}

async fn hello(hello: Hello) {
    println!("Connection opened {:?}", hello.heartbeat_interval)
}

async fn heartbeat_refreshed(_text: Event) {
    println!("Heartbeat refreshed")
}

async fn on_ready(ready: Ready) {
    println!("READY: {:?}", ready.user.username)

    // let _data = serde_json::to_string_pretty(&text)
    //     .unwrap()
    //     .replace("\\", "");

    // let mut _file = File::create("data.json").unwrap();
    // _file.write_all(_data.as_bytes()).unwrap();
}

async fn on_voice_state_update(state: VoiceStateUpdate) {
    match state.channel_id.as_deref().unwrap_or_default() {
        "" => println!("TODO Leave function"),
        "1066465107665748132" => voice::on_voice_channel_create(state).await,
        _ => return,
    }

    // if ()
}

async fn on_guild_create(_text: Value) {
    // println!("READY: {:?}", ready.user.username)
    let _data = serde_json::to_string_pretty(&_text)
        .unwrap()
        .replace("\\", "");

    let mut _file = File::create("dataGC.json").unwrap();
    _file.write_all(_data.as_bytes()).unwrap();
}

async fn on_message(_ctx: Value) {
    // println!("{:?}")
}

// async fn on_

#[tokio::main]
async fn main() {
    dotenv().ok();

    let (mut send, mut read) = connect().await.unwrap().split();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

    let tx_clone = tx.clone();
    tokio::task::spawn(async move {
        loop {
            let message = rx.recv().await.unwrap();
            // println!("{:?}", message);
            send.send(message).await.expect("Failed to send to discord");
        }
    });

    let mut heartbeat_interval = interval(Duration::from_secs(30));

    tokio::task::spawn(async move {
        loop {
            heartbeat_interval.tick().await;
            let heartbeat = r#"{
    			"op": 1,
    			"d": null
    		}"#;

            tx_clone
                .send(Message::Text(heartbeat.to_string()))
                .await
                .expect("Hmm");
            // socket.write_message(Message::text(heartbeat)).unwrap();
        }
    });

    tokio::task::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(message) => {
                    tokio::task::spawn(event_generater(message));
                }
                Err(e) => {
                    eprintln!("Error reading message: {:?}", e);
                    break;
                }
            }
        }
    });
    verify(tx.clone()).await;

    loop {}
}
