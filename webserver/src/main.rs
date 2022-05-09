use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    println!("Starting");
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Someone Connected");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let remote = b"GET /remote HTTP/1.1\r\n";

    let (status_line, filename) =
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "website/index.html")
        }
        else if buffer.starts_with(remote){
            ("HTTP/1.1 200 OK", "website/remote.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND", "website/404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
