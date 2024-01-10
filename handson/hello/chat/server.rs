use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // let addr = "localhost:8888";
    let addr = "127.0.0.1:8888";

    // スレッド間通信。
    let (tx, rx) = mpsc::channel::<String>();

    // クライアント一覧を保持する。
    let mut clients: Vec<TcpStream> = Vec::new();

    let server = TcpListener::bind(addr).expect("Failed to bind");
    server
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");
    println!("listening on {}", addr);

    loop {
        // クライアントの待ち受け。
        if let Ok((client, addr)) = server.accept() {
            println!("{} is connected", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        // スレッド間通信の受信。
        if let Ok(line) = rx.try_recv() {
            println!("send all {}", line);
            clients = send_all(clients, &line);
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);

    // println!("start thread");

    // ここに loop が入ることあるんや、
    thread::spawn(move || loop {
        let mut line = String::new();
        // **改行コードまで読み込む！**
        // クライアント側でメッセージを送信するとき、改行コードを付けて送信すること。
        if let Ok(n) = reader.read_line(&mut line) {
            if n > 0 {
                tx.send(line).expect("Failed to send");
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];

    for mut socket in clients.into_iter() {
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("Write error: {}", e);
            continue;
        }
        // 所有権を回収する。
        collector.push(socket);
    }
    // 所有権を返す。
    collector
}
