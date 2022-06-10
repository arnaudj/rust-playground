use log::error;
use log::info;
use std::io::prelude::*;
use std::net::TcpListener;

pub fn run() {
    let addr = "127.0.0.1:8081";
    let listener = TcpListener::bind(addr).unwrap();
    info!("Listening on {addr}");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    info!("Connection established! {stream:?}");
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(n) => {
            println!("< Read {n} bytes: {}", String::from_utf8_lossy(&buffer[..]));

            let mut content = "HTTP/1.1 404 Not Found\r\n".to_string();
            if let Some(reply) = handle_index(&buffer) {
                content = reply;
            }

            let nb_written = stream.write(content.as_bytes()).unwrap();
            println!("> Written {nb_written} bytes: {}", content);
            stream.flush().unwrap();
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}

fn handle_index(buffer: &[u8]) -> Option<String> {
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let contents = r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Index</title>
    </head>
    <body>
        <h1>Index</h1>
        <p>Hello from Rust</p>
    </body>
</html>"#;

        Some(format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
