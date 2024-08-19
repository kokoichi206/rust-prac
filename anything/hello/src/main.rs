use std::{collections::HashMap, rc::Rc};

use hello::{code_block, header, ulist};

fn main() {
    let content = [
        header!("Hello"),
        code_block!(lang = "go", "fmt.Println(\"Hello, World!\")"),
        ulist!("Hello", "World", "Rust"),
    ]
    .join("\n\n");

    println!("{}", content);

    struct Person {
        name: Option<String>,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Pien".to_string()),
        birth: 1525,
    });
    let first_name = composers[0].name.take();
    print!("First name: {:?}", first_name);
    println!("Birth year: {:?}", composers[0].birth);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    let second = v.swap_remove(2);
    println!("Second element: {}", second);
    println!("Vector: {:?}", v);

    let s: Rc<String> = Rc::new("Xxxx".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
    // s.push_str("pien");
    println!("s: {}, t: {}, u: {}", s, t, u);

    // 借用
    type Table = HashMap<String, Vec<String>>;
    fn show(table: &Table) {
        // &String, &Vec<String>
        // ループの変数も参照で受け取るようになっている
        for (artist, works) in table {
            println!("works by {}: ", artist);
            for work in works {
                println!("  {}", work);
            }
        }
    }
    fn showVal(table: Table) {
        // String, Vec<String>
        for (artist, works) in table {
            println!("works by {}: ", artist);
            for work in works {
                println!("  {}", work);
            }
        }
    }
    // 可変参照
    fn sort_works(table: &mut Table) {
        for (_artist, works) in table {
            works.sort()
        }
    }
    let mut table = Table::new();
    table.insert("Pi".to_string(), vec!["afa".to_string(), "fea".to_string()]);
    table.insert(
        "Piyo".to_string(),
        vec!["wwww".to_string(), "fea".to_string()],
    );
    table.insert("ABE".to_string(), vec!["bbb".to_string(), "aa".to_string()]);
    show(&table);
    println!("table: {:?}", table);
    sort_works(&mut table);
    println!("table: {:?}", table);

    let x = 10;
    let y = 20;
    let mut r = &x;
    r = &y;
    println!("r: {}", r);
    println!("x: {}", x);
    println!("y: {}", y);
    let xx = &x;
    let xxx = &xx;
    println!("xxx: {}", xxx);
    let y2 = 10;
    let yy = &y2;
    assert!(xx <= yy);
    assert!(xx == yy);

    fn factorial(n: usize) -> usize {
        (1..n + 1).fold(1, |a, b| a * b)
    }
    let r = &factorial(6);
    // 参照同士の加算も可能！？
    assert_eq!(r + &1000, 1720);

    let s = smallest(&[1, 2, 3, 4, -100, 5]);
    println!("s: {}", s);
}

static mut STASH: &i32 = &128;
// fn f(p: &i32) {
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if r < s {
            s = r;
        }
    }
    s
}

struct S<'a> {
    r: &'a i32,
}

// struct SXY<'a> {
struct SXY<'a, 'b> {
    x: &'a i32,
    // y: &'a i32,
    y: &'b i32,
}

fn sxy() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            // s.y を &y (y の参照) で初期化しているので、
            // 'a は y の生存期間よりも長くできない。
            let s = SXY { x: &x, y: &y };
            r = s.x;
        }
    }
}
