use std::{io::Write, net};

const CONNECT_ADDRESS: &str = "127.0.0.1:8001";

fn main() {
    let mut stream =
        net::TcpStream::connect(CONNECT_ADDRESS).expect("Couldn't establish TCP connection");

    stream.write_all("$> ".as_bytes()).unwrap();
}
