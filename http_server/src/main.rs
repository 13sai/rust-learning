use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("conn success!");

        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let content = fs::read_to_string("sai.html").unwrap();

        // println!("Req: {}", content);
        let res = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );

        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let line = "HTTP/1.1 404 NOT FOUND\r\n";
        let content = fs::read_to_string("404.html").unwrap();
        let res = format!("{}Content-Length: {}\r\n\r\n{}", line, content.len(), content);
        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
