use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub mod thread_pool;

const HELLO_HTML: &str = include_str!("../web/hello.html");
const NOT_FOUND_HTML: &str = include_str!("../web/404.html");

fn response_for_request(request_line: &str) -> (&'static str, &'static str) {
    match request_line.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["GET", "/", "HTTP/1.1"] => ("200 OK", HELLO_HTML),
        ["GET", "/sleep", "HTTP/1.1"] => ("200 OK", HELLO_HTML),
        _ => ("404 NOT FOUND", NOT_FOUND_HTML),
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let request_line = {
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => return,
            Ok(_) => line,
        }
    };

    println!("Request: {}", request_line.trim());

    if request_line.trim() == "GET /sleep HTTP/1.1" {
        thread::sleep(Duration::from_secs(5));
    }

    let (status, contents) = response_for_request(request_line.trim());
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nConnection: close\r\n\r\n{contents}",
        contents.len()
    );

    if let Err(error) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_route_returns_success_page() {
        let (status, body) = response_for_request("GET / HTTP/1.1");
        assert_eq!(status, "200 OK");
        assert!(body.contains("Hello from Rust!"));
    }

    #[test]
    fn sleep_route_returns_success_page() {
        let (status, body) = response_for_request("GET /sleep HTTP/1.1");
        assert_eq!(status, "200 OK");
        assert!(body.contains("Hello from Rust!"));
    }

    #[test]
    fn unknown_route_returns_not_found_page() {
        let (status, body) = response_for_request("GET /missing HTTP/1.1");
        assert_eq!(status, "404 NOT FOUND");
        assert!(body.contains("404 - Page Not Found"));
    }

    #[test]
    fn unsupported_method_returns_not_found() {
        let (status, _) = response_for_request("POST / HTTP/1.1");
        assert_eq!(status, "404 NOT FOUND");
    }
}
