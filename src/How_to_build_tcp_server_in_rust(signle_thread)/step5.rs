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
    stream.set_nodelay(true).expect("Can't delay!");

    if buffer.starts_with(get){
        let contents = fs::read_to_string("hello.html").unwrap();
        let size = contents.len();
        let response = format!("HTTP/1.1 200 OK\r\nServer: Apache\r\nContent-Type: text/plain;Content-Length: {}; charset=UTF-8\r\n\r\n{}",size,contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("{}{}",status_line,contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}