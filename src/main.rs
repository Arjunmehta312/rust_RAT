use std::process::{Command, Stdio};
use std::io;
use std::io::Write;

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
        let mut command: String = String::new();
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut command).expect("Failed to read line");

        let command = command.trim(); // Remove trailing newline

        if command == "alvida" {
            break;
        }

        let result = execute_command(command); // Pass trimmed input
        println!("{}", result);
    }
}
