use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8989").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "sai.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();

    let res = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        line,
        content.len(),
        content
    );

    stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
}
