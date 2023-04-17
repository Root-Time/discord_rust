use std::sync::mpsc::{self, Receiver, Sender};

use chrono;

use tungstenite::{connect, Message};
use url::Url;

#[tokio::main]
async fn main() {
    let _token = "ODI4NjA3Mjk5NTIyMTM0MDY2.G90aph.FUvPInf-Vho2yAvuPybXI2xBkfeVM-vjcY5dN0";

    println!("Started {:?}", chrono::offset::Local::now());
    let (mut socket, _response) =
        connect(Url::parse("wss://gateway.discord.gg/").unwrap()).expect("Can't connect");

    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let mess = socket.read_message().unwrap();
    println!("{:?}", mess);

    // let mut heartbeat_interval = interval(Duration::from_secs(10));

    let tx_clone = tx.clone();
    std::thread::spawn(move || loop {
        let message = rx.recv().unwrap();
        // println!("{:?}", message);
        socket.write_message(message);
    });

    loop {
        let message = socket.read_message().unwrap();
        println!("Received: {:?}", message);

        // Send the received message to the other thread for processing
        tx_clone.send(message).unwrap();
    }
    // tokio::task::spawn(async move {
    //     loop {
    //         heartbeat_interval.tick().await;
    //         let heartbeat = r#"{
    // 			"op": 1,
    // 			"d": null
    // 		}"#;

    //         socket.write_message(Message::text(heartbeat)).unwrap();

    //         // let mess = socket.read_message().unwrap();
    //         let mess = rx.recv().unwrap();
    //         println!("{:?}", mess);
    //     }
    // });

    // loop {}
    // let identifier = format!(
    //     r#"{{
    // 	"op": 2,
    // 		"d": {{
    // 			"token": "{token}",
    // 			"intents": 513,
    // 			"properties": {{
    // 			"os": "linux",
    // 			"browser": "my_library",
    // 			"device": "my_library"
    // 			}}
    // 		}}
    // 	}}"#
    // );

    // socket.write_message(Message::text(identifier)).unwrap();
}
