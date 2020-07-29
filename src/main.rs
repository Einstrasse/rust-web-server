use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) {
    println!("Connection!");
    let mut buf = [0; 500];
    let data = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=iso-8859-1\r\nConnection: close\r\nContent-Length: 14\r\n\r\nHello world!\r\n";
    // stream.read(&buf).expect("Receive error");
    stream.read(&mut buf).expect("Receive error");
    println!("Received HTTP Request...\n{}", String::from_utf8_lossy(&buf[..]));
    stream.write(data.as_bytes()).expect("Response error");
    
}

fn main() -> std::io::Result<()> {
    let server = "127.0.0.1:1024";
    let listener = TcpListener::bind(server)?;

    // accept connections and process them serially
    println!("Server is running on {}...", server);
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}