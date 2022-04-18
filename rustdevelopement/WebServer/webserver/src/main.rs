use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;

fn main()
{
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming()
    {
        let stream = stream.unwrap();
        handle_connection(stream);
        //println!("Conn Succeeded");
    }
}
// I guess in Rust by default all is immutable?
// This is mutable here because internal state of stream can keep changing.
fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK \r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}