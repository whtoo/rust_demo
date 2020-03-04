use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main()  {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\nContent-Length: ","hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\nContent-Length: ","404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let size = contents.len();
    let response = format!("{} {}\r\n\r\n{}",status_line,size,contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    
}