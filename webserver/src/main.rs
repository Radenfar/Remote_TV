use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    println!("Starting");
    let listener = TcpListener::bind("localhost:5000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Someone Connected");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));

    // pages
    let get = b"GET / HTTP/1.1\r\n";
    let remote = b"GET /remote.html HTTP/1.1\r\n";
    let downloads = b"GET /downloads.html HTTP/1.1\r\n";

    // css
    let index_css = b"GET /css/index.css HTTP/1.1\r\n";
    let download_css = b"GET /css/download.css HTTP/1.1\r\n";
    let remote_css= b"GET /css/remote.css HTTP/1.1\r\n";

    let (status_line, filename, content_type) =

        // html
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "website/index.html", "text/html; charset=iso-8859-1")
        } else if buffer.starts_with(remote){
            ("HTTP/1.1 200 OK", "website/remote.html", "text/html; charset=iso-8859-1")
        } else if buffer.starts_with(downloads) {
            ("HTTP/1.1 200 OK", "website/downloads.html", "text/html; charset=iso-8859-1")
        }

        // css
        else if buffer.starts_with(index_css) {
            ("HTTP/1.1 200 OK", "website/css/index.css", "text/css; charset=iso-8859-1")
        }
        else if buffer.starts_with(download_css) {
            ("HTTP/1.1 200 OK", "website/css/download.css", "text/css; charset=iso-8859-1")
        }
        else if buffer.starts_with(remote_css) {
            ("HTTP/1.1 200 OK", "website/css/remote.css", "text/css; charset=iso-8859-1")
        }

        // 404 html
        else {
            ("HTTP/1.1 404 NOT FOUND", "website/404.html", "text/html; charset=iso-8859-1")
        };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content_type,
        contents.len(),
        contents);

    //println!("{}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
