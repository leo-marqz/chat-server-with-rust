
use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use chrono::prelude::*;
use tokio::sync::broadcast;

const HOST: &str = "127.0.0.1:7000"; // The server address

#[tokio::main] // This attribute is used to run the async main function
async fn main() {

    println!("Server listening on: {}", HOST);

    let mut message_counter: i32 = 0;
    let server_listener = TcpListener::bind(HOST).await.unwrap();
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);

    loop {
        
        let (mut socket, mut _address ) = server_listener.accept().await.unwrap();
        
        println!("\n::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        println!(":::::::::: Accepted connection from: {} ::::::::::", _address);
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::\n");

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            
            let (reader, mut writer) = socket.split();
            let mut buffer_reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    _ = buffer_reader.read_line(&mut line) => {
                        let datetime = Local::now().format("%H:%M:%S");
                        line = format!("\n|{}|RustChat|{}|-> {}", datetime, message_counter, line);
                        println!("{}", line);
                        tx.send( (line.clone(), _address) ).unwrap();
                        line.clear();
                        message_counter += 1;
                    },
                    result = rx.recv() => {
                        let (message, message_address) = result.unwrap();
                        if message_address != _address {
                            writer.write_all(message.as_bytes()).await.unwrap();
                        }
                    }
                }
            }

        });

    }

    

}
