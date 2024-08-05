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
        for (artist, works) in table {
            println!("works by {}: ", artist);
            for work in works {
                println!("  {}", work);
            }
        }
    }
    let mut table = Table::new();
    table.insert("Pi".to_string(), vec!["afa".to_string(), "fea".to_string()]);
    table.insert("Piyo".to_string(), vec!["afa".to_string(), "fea".to_string()]);
    show(&table);
    println!("table: {:?}", table);
}
