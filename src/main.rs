use std::net::{TcpListener, TcpStream};
use std::io::{Write};

fn handle_client(mut stream: TcpStream) {
    println!("Connection!");
    let data = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=iso-8859-1\r\nConnection: close\r\nContent-Length: 14\r\n\r\nHello world!\r\n";
    stream.write(data.as_bytes()).expect("Response error");
    
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}