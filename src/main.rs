use std::{io::Read, io::Write, net::TcpListener, net::TcpStream, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 256];
    loop {
        stream.read(&mut buf).unwrap();
        stream.write(b"+PONG\r\n").unwrap();
    }
}
