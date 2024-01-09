use std::collections::HashMap;
use std::io::{self, Write};

const V_DATA: &str = "C,C,A,A,B,C,B,C";

fn input(prompt: &str) -> f64 {
    // println!("{}", prompt);
    print!("{}", prompt);
    io::stdout().flush().expect("フラッシュに失敗");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("入力エラー");
    // Result 型から実際の値を取り出すために expect を使う。
    return input.trim().parse().expect("数値変換エラー");
}

fn sum_slice(items: &[i32]) -> i32 {
    let mut sum = 0;
    for i in items {
        sum += i;
    }
    return sum;
}

// tuple を構造体として定義する。
// 文字列は &str 型ではなく String 型を使う。
struct Person(String, u32);

// シンプルな構造体。
// 上のタプルのレイトどう違う？？
#[derive(Debug)]
struct CarSpec {
    maker: String,
    model: String,
    year: u32,
    color: String,
}

#[derive(Debug)]
struct Car {
    spec: CarSpec,
    price: u32,
    distance: u32,
}

impl Car {
    // 引数の self なしで Self が返る。
    fn new(spec: CarSpec, price: u32) -> Self {
        Car {
            spec: spec,
            price: price,
            distance: 0,
        }
    }

    fn drive(&mut self, distance: u32) {
        self.distance += distance;
    }

    fn spec(&self) -> String {
        format!(
            "{} {} ({}) {}",
            self.spec.maker, self.spec.model, self.spec.year, self.spec.color
        )
    }
}

impl Printable for Car {
    fn print(&self) {
        println!("car: {}", self.spec());
    }
}

trait Printable {
    fn print(&self);
}

fn print_something(p: &impl Printable) {
    p.print();
}

// fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

struct PrimeIterator {
    n: u8,
}

impl PrimeIterator {
    fn new() -> Self {
        PrimeIterator { n: 1 }
    }
    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 {
                return false;
            }
        }
        return true;
    }
}
// implement Iterator trait
// https://doc.rust-lang.org/std/iter/trait.Iterator.html
impl Iterator for PrimeIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.n += 1;
            if std::u8::MAX <= self.n {
                return None;
            }
            if self.is_prime() {
                // what is the type of Some()?
                // https://doc.rust-lang.org/std/option/enum.Option.html
                return Some(self.n);
            }
        }
    }
}

enum Coin {
    Coin1(isize),
    Coin5(isize),
    Coin10(isize),
}
impl Coin {
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Coin1(n) => n,
            Coin::Coin5(n) => n * 5,
            Coin::Coin10(n) => n * 10,
        }
    }
}

fn fizzbuzz() {
    for i in 1..=100 {
        // この match 文おもろい。
        // match (i % 3, i % 5) {
        //     (0, 0) => println!("FizzBuzz"),
        //     (0, _) => println!("Fizz"),
        //     (_, 0) => println!("Buzz"),
        //     (_, _) => println!("{}", i),
        // }

        // match guard
        let msg = match i {
            n if n % 15 == 0 => "FizzBuzz".to_string(),
            n if n % 3 == 0 => "Fizz".to_string(),
            n if n % 5 == 0 => "Buzz".to_string(),
            _ => i.to_string(),
        };
        println!("{}", msg);
    }
}

// これが必要らしい。
mod random;
use random::{linear, xorshift};
fn main() {
    let mut seed = 1u32;
    let r1 = linear::rand(&mut seed);
    let r2 = xorshift::rand(&mut seed);
    println!("r1: {}, r2: {}", r1, r2);

    let wallet: Vec<Coin> = vec![Coin::Coin1(3), Coin::Coin5(2), Coin::Coin10(1)];
    let total = wallet.iter().fold(0, |sum, n| sum + n.calc_price());
    println!("total: {}", total);

    let prime_iter = PrimeIterator::new();
    for n in prime_iter {
        print!("{} ", n);
    }

    println!("{}", add(1, 2));
    println!("{}", add(1.1, 2.2));
    println!("{}", add::<i32>(1, 2));
    // error in my environment.
    // println!("{}", add('a', 'b'));

    let cs = CarSpec {
        maker: "Toyota".to_string(),
        model: "Prius".to_string(),
        year: 2015,
        color: "blue".to_string(),
    };
    let mut carInstance = Car::new(cs, 1000);
    carInstance.drive(1000);
    println!("car: {}", carInstance.spec());

    print_something(&carInstance);

    // tuple
    let p = Person("Taro".to_string(), 20);
    println!("name: {}, age: {}", p.0, p.1);

    let s = String::from("Hello Rust");
    // slice を作成する。
    // s1 は &str 型。
    let s1 = &s[0..5];
    println!("s1: {}", s1);

    let car = CarSpec {
        maker: "Toyota".to_string(),
        model: "Prius".to_string(),
        year: 2015,
        color: "blue".to_string(),
    };
    println!(
        "car: {} {} ({}) {}",
        car.maker, car.model, car.year, car.color
    );

    // from array
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = sum_slice(&a[..]);
    println!("sum: {}", sum);

    // from vector
    let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = sum_slice(&b[..]);
    println!("sum: {}", sum);

    // コマンドライン引数を得る。
    let args = std::env::args();
    let mut total = 0.0;
    for (i, s) in args.enumerate() {
        // 0 番目の引数は実行ファイル名なので飛ばす。
        if i == 0 {
            continue;
        }
        let n: f64 = match s.parse() {
            Ok(n) => n,
            Err(_) => 0.0,
        };
        total += n;
    }
    println!("total: {}", total);

    // ベクター型 ⇨ サイズ可変な配列
    let nums = vec![1, 2, 3, 4, 5];
    println!("nums: {:?}", nums);

    let mut c_map = HashMap::new();
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);
    for w in V_DATA.split(",") {
        c_map.insert(w, c_map[w] + 1);
    }
    for k in c_map.keys() {
        // get の戻り値は Option 型。
        // let v = c_map.get(k);
        println!("{}: {}", k, c_map[k]);
    }

    let height_cm = input("height (cm): ");
    let weight = input("weight (kg): ");
    let height = height_cm / 100.0;
    let bmi = weight / (height * height);
    println!("BMI: {}", bmi);
}
