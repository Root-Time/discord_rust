use std::time::Duration;

use dotenv::dotenv;
use futures::{SinkExt, StreamExt};
use serde_json::Value;
use std::env;
use tokio::{sync::mpsc::Sender, time::interval};
// use futures::channel::mpsc::Receiver;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Message},
};
// use tungstenite::{connect, Message};
use url::Url;

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
    			"intents": 513,
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
    let json: Value = serde_json::from_str(&text).unwrap();

    println!("{:?}", text);
}

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
            send.send(message).await.expect("Test");
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
