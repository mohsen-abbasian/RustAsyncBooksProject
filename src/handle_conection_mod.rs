use std::fs;
use std::time::Duration;
use std::io::prelude::*;
use std::net::TcpStream;
use async_std::task;

pub async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/html_codes/hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/html_codes/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./src/html_codes/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
