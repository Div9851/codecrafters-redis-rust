use std::{io::Read, io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => loop {
                let mut buf = [0; 256];
                _stream.read(&mut buf).unwrap();
                _stream.write(b"+PONG\r\n").unwrap();
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
