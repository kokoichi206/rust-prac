use std::time::Instant;

// recursive fibonacci without multithreading.
fn fib(n: i64) -> i64 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!(
        "{}.{:03} seconds",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
}

fn single() {
    let request_nums = [43, 20, 30, 40, 36, 37, 39];
    let start_time = Instant::now();
    for num in request_nums {
        let answer = fib(num);
        println!("fib({}) = {}", num, answer);
    }
    show_time(start_time);
}

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn multithreading() {
    let request_nums = [43, 20, 30, 40, 36, 37, 39];
    let start_time = Instant::now();

    let (tx, rx) = mpsc::channel::<(i64, i64)>();
    for num in request_nums {
        let sender = tx.clone();
        // ここは move がないとコンパイルエラーになる。
        // cannot be shared between threads safely.
        thread::spawn(move || {
            let answer = fib(num);
            sender.send((num, answer)).unwrap();
        });
    }

    let mut job = request_nums.len();
    loop {
        if let Ok((arg, answer)) = rx.recv() {
            job -= 1;
            println!("fib({}) = {}", arg, answer);

            if job == 0 {
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    show_time(start_time);
}

fn main() {
    // single();
    multithreading();
}
