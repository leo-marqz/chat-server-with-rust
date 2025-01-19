
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use chrono::prelude::*;
use tokio::sync::broadcast;

const HOST: &str = "127.0.0.1:8080"; // The server address

#[tokio::main] // This attribute is used to run the async main function
async fn main() {

    // Print the server address
    println!("Server listening on: {}", HOST);

    // Create a message counter
    let mut message_counter: i32 = 0;
    
    // Create a TCP listener bound to the server address
    let server_listener = TcpListener::bind(HOST).await.unwrap();

    // Create a broadcast channel: 10 is the buffer size
    let (tx, _rx) = broadcast::channel::<String>(10);

    // Spawn a new task to handle the incoming connections
    loop {
        
        // Accept incoming connections
        let (mut socket, mut _address ) = server_listener.accept().await.unwrap();
    
        // Clone the broadcast channel
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // Spawn a new task to handle the incoming connection
        tokio::spawn(async move {
            
            // Split the socket into reader and writer
            let (reader, mut writer) = socket.split();
        
            println!("Accepted connection from: {}", _address);
        
            // Create a buffer reader
            let mut buffer_reader = BufReader::new(reader);
        
            // Create a buffer to store the incoming data
            let mut line = String::new();

            
            loop {
                // Read the incoming data
                let _bytes = buffer_reader.read_line(&mut line).await.unwrap();
                
                // If the incoming data is empty, break the loop
                if _bytes == 0 {
                    println!("Connection closed...............................");
                    break;
                }

                // Send the broadcast message
                tx.send( line.clone() ).unwrap();

                let mut message = rx.recv().await.unwrap();
                
                // Response message
                let datetime = Local::now().format("%Y-%m-%d %H:%M:%S");
                message = format!("\n|{} | Message [{}]: {}", datetime, message_counter, line); 
        
                println!("{}", message);
                
                // Print the incoming data
                writer.write_all(message.as_bytes() ).await.unwrap();
        
                // Clear the buffer
                line.clear();
        
                // Increment the message counter
                message_counter += 1;
        
            }
        });
    }

    

}
