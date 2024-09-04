use async_std::io::{Read, Write};
use async_std::prelude::*;
use std::fs;
use std::time::Duration;
// use async_std::net::TcpStream;
use async_std::task;
use crate::MockTcpStream;

pub async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/html_codes/hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/html_codes/hello.html")
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            "./src/html_codes/404.html",
        )
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}{contents}");

    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    handle_connection(&mut stream).await;

    let expected_contents = fs::read_to_string("./src/html_codes/hello.html").unwrap();
    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}