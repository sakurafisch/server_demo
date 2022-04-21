use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("./src/hello.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("./src/404.html").unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    let pool = server_demo::ThreadPool::new(16);
    if let Err(e) = listener {
        println!("Error binding TCP listener:\n{}", e);
    } else if let Ok(l) = listener {
        for stream in l.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    }
    println!("Shutting down");
}
