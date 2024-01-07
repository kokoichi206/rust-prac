use num_bigint::BigInt;

fn main() {
    let msg_list = ["おはようございます", "こんにちは"];
    for msg in msg_list {
        println!("{}", msg);
    }

    let v = BigInt::from(1234);
    println!("{}", v.pow(5678))
}
