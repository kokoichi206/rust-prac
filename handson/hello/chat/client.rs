use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // let addr = "localhost:8888";
    let addr = "127.0.0.1:8888";

    // サーバーとの接続を確立する。
    let mut socket = TcpStream::connect(addr).expect("Failed to connect");
    socket
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");
    println!("Connected to {}", addr);

    // 受信ようスレッド。
    start_thread(socket.try_clone().unwrap());

    // 標準入力からのメッセージ送信。
    let user = input("what's your name?: ");
    println!("Welcome {}!", user);
    loop {
        let line = input("> ");
        if line == ":quit" || line == ":q" {
            break;
        }
        let line = format!("{} > {}\n", user, line);
        let buf = line.as_bytes();
        socket.write_all(buf).unwrap();
    }
}

fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        // サーバーからのメッセージ受信。
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 {
                println!("received: {}", buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

// 標準入力から文字列を取得する。
fn input(msg: &str) -> String {
    // println!("{}", msg);
    if msg != "" {
        print!("{}", msg);
    }
    std::io::stdout().flush().unwrap();

    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Failed to read from stdin");
    String::from(buf.trim())
}
