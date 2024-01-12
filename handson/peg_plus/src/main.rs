peg::parser! (grammar calc() for str {
    // ルートとなる規則。
    pub rule eval() -> i64
    = term()

    // 足し算。
    rule term() -> i64
    = v1:num() "+" v2:num()
    { v1 + v2 }

    rule num() ->  i64
    = value:$(['0'..='9']+)
    { value.parse().unwrap() }
});

fn main() {
    println!("1+2={}", calc::eval("1+2").unwrap());
}
