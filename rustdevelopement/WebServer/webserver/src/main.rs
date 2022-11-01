use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        //println!("Conn Succeeded");
    }
}
// I guess in Rust by default all is immutable?
// This is mutable here because internal state of stream can keep changing.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        // handle response
        let contents = fs::read_to_string("hello.html").unwrap();
        // add sleep to simulate slow response
        std::thread::sleep(std::time::Duration::from_secs(5));
        let response = format!(
            "HTTP/1.1 200 OK \r\n Content-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        //let response = "HTTP/1.1 200 OK \r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    } else {
        //some other request return 404    
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

