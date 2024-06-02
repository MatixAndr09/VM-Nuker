use std::io::Read;
use ssh2::Session;

fn main() {
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