fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

// & -> 値の参照
// mut -> 可変
fn get_primes(primes: &mut [usize; 100]) {
    let mut i = 2;
    let mut count = 0;
    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    println!("usize::MAX = {}", usize::MAX);

    // 初期値0で100個の要素を持つ配列を作成。
    let mut primes = [0; 100]
    // 可変参照な値を引数に指定して関数を呼び出す場合、呼び出し側でも可変であることを示さないといけない！！！
    get_primes(&mut primes);
    // :? などとすると、データの型に応じて出力形式を変えてくれる。
    println!("{:?}", primes);
}
