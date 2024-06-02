use std::process::Command;
use std::io::Read;
use ssh2::Session;

fn fire(text: &str) -> String {
    let _ = Command::new("clear").status();

    let mut faded = String::new();
    let mut green = 250;

    for line in text.lines() {
        faded.push_str(&format!("\x1b[38;2;255;{};0m{}\x1b[0m\n", green, line));
        if green > 0 {
            green -= 25;
        }
    }
    faded
}

fn main() {
    let message = r#"
      ██▒   █▓ ███▄ ▄███▓    ███▄    █  █    ██  ██ ▄█▀▓█████  ██▀███  
     ▓██░   █▒▓██▒▀█▀ ██▒    ██ ▀█   █  ██  ▓██▒ ██▄█▒ ▓█   ▀ ▓██ ▒ ██▒
      ▓██  █▒░▓██    ▓██░   ▓██  ▀█ ██▒▓██  ▒██░▓███▄░ ▒███   ▓██ ░▄█ ▒
       ▒██ █░░▒██    ▒██    ▓██▒  ▐▌██▒▓▓█  ░██░▓██ █▄ ▒▓█  ▄ ▒██▀▀█▄  
        ▒▀█░  ▒██▒   ░██▒   ▒██░   ▓██░▒▒█████▓ ▒██▒ █▄░▒████▒░██▓ ▒██▒
        ░ ▐░  ░ ▒░   ░  ░   ░ ▒░   ▒ ▒ ░▒▓▒ ▒ ▒ ▒ ▒▒ ▓▒░░ ▒░ ░░ ▒▓ ░▒▓░
        ░ ░░  ░  ░      ░   ░ ░░   ░ ▒░░░▒░ ░ ░ ░ ░▒ ▒░ ░ ░  ░  ░▒ ░ ▒░
          ░░  ░      ░         ░   ░ ░  ░░░ ░ ░ ░ ░░ ░    ░     ░░   ░ 
           ░         ░               ░    ░     ░  ░      ░  ░   ░     
          ░                                  
                                       ＢＹ ＭＡＴＩＸＡＮＤＲ０９                     
"#;

    let fire_text = fire(message);
    println!("{}", fire_text);

    let user = "USERNAME HERE";
    let host = "HOST HERE (IP or domain)";
    let passwd = "PASSWORD TO USER HERE";

    let tcp = std::net::TcpStream::connect(format!("{}:22", host)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(user, passwd).unwrap();
    let mut channel = sess.channel_session().unwrap();

    channel.exec("ls").unwrap(); // Runs a ls command
    let mut output = Vec::new();

    channel.read_to_end(&mut output).unwrap();
    println!("{}", String::from_utf8_lossy(&output));
}