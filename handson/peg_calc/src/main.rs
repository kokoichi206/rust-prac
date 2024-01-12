peg::parser! (grammar calc() for str {
    // root rule
    pub rule eval() -> i64
    = expr()

    // plus, minus
    rule expr() -> i64
    = l:term() "+" r:expr() { l + r }
    / l:term() "-" r:expr() { l - r }
    / term()

    // multiply, divide
    rule term() -> i64
    = l:value() "*" r:term() { l * r }
    / l:value() "/" r:term() { l / r }
    / v:value()

    rule value() -> i64
    = number()
    / "(" v:expr() ")" { v }

    rule number() -> i64
    = n:$(['0'..='9']+) { n.parse().unwrap() }
});

fn main() {
    println!("Hello, world!");
}
