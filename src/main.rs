use std::process::{Command, Stdio};

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
    let command = "echo 'You have been hacked' > test.txt"; // Replace with any shell command
    let result = execute_command(command);
    println!("{}", result);
}
