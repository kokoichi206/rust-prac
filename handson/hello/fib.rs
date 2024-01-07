fn main() {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..30 {
        let c = a + b;
        println!("{}", c);
        a = b;
        b = c;
    }
}
