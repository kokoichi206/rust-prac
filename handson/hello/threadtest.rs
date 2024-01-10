use std::{thread, time};

fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{}: {}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    println!("--- without threads ---");
    sleep_print("without threads");

    println!("--- with threads ---");
    let handle = thread::spawn(|| {
        sleep_print("with thread 1");
    });
    thread::spawn(|| {
        sleep_print("with thread 2");
    });
    
    sleep_print("main thread  ");
}
