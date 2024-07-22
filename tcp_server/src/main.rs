use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
// use std::thread::spawn;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream
        .read(&mut buffer)
        .expect("failed to read from client.");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("req: {}", request);

    let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!".as_bytes();
    stream.write(response).expect("failed to write to client.");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9911").expect("failed to bind to address");
    println!("Listening on port 9911");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("failed to establish a connection: {}", e);
            }
        }
    }
}
