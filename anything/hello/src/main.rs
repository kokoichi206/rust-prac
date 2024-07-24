use hello::{code_block, header, ulist};

fn main() {
    let content = [
        header!("Hello"),
        code_block!(lang = "go", "fmt.Println(\"Hello, World!\")"),
        ulist!("Hello", "World", "Rust"),
    ]
    .join("\n\n");

    println!("{}", content);
}
