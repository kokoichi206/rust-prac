use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    for i in 1..=3 {
        let msg = format!("{}: {}", name, i);
        sender.send(msg).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
    sender.send("done".to_string()).unwrap();
}

fn main() {
    // Create a channel for sending strings.
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a thread to send messages.
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("John", sender);
    });

    // Spawn a thread to send messages.
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("Doe", sender);
    });

    // Receive messages.
    loop {
        let buf = rx.recv().unwrap();
        println!("{}", buf);
        if buf == "done" {
            break;
        }
    }
}
