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
struct CarSpec {
    maker: String,
    model: String,
    year: u32,
    color: String,
}

fn main() {
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
    println!("car: {} {} ({}) {}", car.maker, car.model, car.year, car.color);

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
