fn main() {
    // sec2p2();
    // sec2p3();
    // sec2p4();
    sec2p5();
}

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)] // トレイトの継承
struct Vm {
    stack: Vec<Value>,
    vars: Vec<HashMap<String, Value>>,
    // 波括弧によるブロック定義の進捗を表す！
    blocks: Vec<Vec<Value>>,
}

impl Vm {
    fn new() -> Self {
        // 参照用の定義。
        let functions: [(&str, fn(&mut Vm)); 10] = [
            ("+", add),
            ("-", sub),
            ("*", mul),
            // ("/", div),
            ("<", lt),
            ("if", op_if),
            ("def", op_def),
            ("puts", puts),
            ("pop", pop),
            ("dup", dup),
            ("exch", exch),
            // ("index", index),
        ];
        Self {
            stack: vec![],
            // vars: HashMap::new(),
            vars: vec![functions
                .into_iter()
                .map(|(name, fun)| (name.to_owned(), Value::Native(NativeOp(fun))))
                .collect()],
            blocks: vec![],
        }
    }

    fn find_var(&self, name: &str) -> Option<Value> {
        self.vars
            .iter()
            .rev()
            .find_map(|vars| vars.get(name).map(|var| var.to_owned()))
    }
}

fn eval(code: Value, vm: &mut Vm) {
    // fn eval(code: Value, stack: &mut Vec<Value>) {
    if let Some(top_block) = vm.blocks.last_mut() {
        top_block.push(code);
        return;
    }
    if let Value::Op(ref op) = code {
        let val = vm
        .find_var(op)
        .expect(&format!("{op:?} is not a defined operation"));
            // .get(op)
            // .expect(&format!("{op:?} is not a defined operation"))
            // .clone();

        match val {
            Value::Block(block) => {
                vm.vars.push(HashMap::new());
                for code in block {
                    // print
                    println!("{:?}", vm.stack);
                    println!("{:?}", vm.vars);
                    // println!("{:?}", vm.blocks);
                    println!("{:?}", code);
                    eval(code, vm);
                }
                vm.vars.pop();
            }
            Value::Native(op) => op.0(vm),
            _ => vm.stack.push(val),
        }
    } else {
        vm.stack.push(code.clone());
    }
}

// duplicate: スタックの最上にある値を複製し push.
// a b c => a b c c
fn dup(vm: &mut Vm) {
    let value = vm.stack.last().unwrap();
    vm.stack.push(value.clone());
}

fn is_commented_line(line: &str) -> bool {
    line.trim_start().starts_with("//")
}

// exchange: スタックの最上位の 2 つの値を交換する。
// a b c => a c b
fn exch(vm: &mut Vm) {
    // b
    let last = vm.stack.pop().unwrap();
    // c
    let second = vm.stack.pop().unwrap();
    // c
    vm.stack.push(last);
    // b
    vm.stack.push(second);
}

// https://doc.rust-lang.org/reference/macros-by-example.html
macro_rules! impl_op {
    {$name:ident, $op:tt} => {
        fn $name(vm: &mut Vm) {
            let rhs = vm.stack.pop().unwrap().as_num();
            let lhs = vm.stack.pop().unwrap().as_num();
            vm.stack.push(Value::Num((lhs $op rhs) as i32))
        }
    }
}
impl_op!(add, +);
impl_op!(sub, -);
impl_op!(mul, *);
impl_op!(lt, <);

fn pop(vm: &mut Vm) {
    vm.stack.pop().unwrap();
}

// fn op_if(stack: &mut Vec<Value>) {
fn op_if(vm: &mut Vm) {
    let false_branch = vm.stack.pop().unwrap().to_block();
    let true_branch = vm.stack.pop().unwrap().to_block();
    let cond = vm.stack.pop().unwrap().to_block();

    for code in cond {
        eval(code, vm);
    }

    let cond_result = vm.stack.pop().unwrap().as_num();

    if cond_result != 0 {
        for cond in true_branch {
            eval(cond, vm);
        }
    } else {
        for code in false_branch {
            eval(code, vm);
        }
    }
}

// /x 10 20 + def
// x := 20 + 10
// x := 20 + 10 + 20 はどう？
// Q.
// /x 20 10 + 20 + def
// /x 20 10 +
// /x 30
// /x 30 20 +
// /x 50
// /x 50 def
fn op_def(vm: &mut Vm) {
    let value: Value = vm.stack.pop().unwrap();
    eval(value, vm);
    let value = vm.stack.pop().unwrap();
    let sym = vm.stack.pop().unwrap().as_sym().to_string();

    vm.vars.last_mut().unwrap().insert(sym, value);
}

#[derive(Debug, Clone, PartialEq, Eq)] // トレイトの継承
enum Value {
    // スタック上に push された数値。
    Num(i32),
    // 演算子。
    Op(String),
    // Symbol 変数。
    Sym(String),
    // ネストされたブロック（？）
    // { } とかで囲まれたところを表す。
    Block(Vec<Value>),
    Native(NativeOp),
}

// newtype パターン
// 関数ポインタを中身に持つタプル構造体。
// 関数ポインタに対する Debug, PartialEq, Eq トレイトを derive 属性で自動的に定義できないから！
// ↑ Orphan rule
// newtype パターンにおいては, value.0 で中身にアクセスする必要がある。
#[derive(Clone)]
struct NativeOp(fn(&mut Vm));

impl PartialEq for NativeOp {
    fn eq(&self, other: &NativeOp) -> bool {
        // *const は生ポインタで、安全性は保証されていない。
        self.0 as *const fn() == other.0 as *const fn()
    }
}

impl Eq for NativeOp {}

// あれ、今なんでこの Debug トレイトを実装する必要があったんだっけ？
// Value が実装してるトレイトだからか、
// https://doc.rust-lang.org/std/fmt/trait.Debug.html
impl std::fmt::Debug for NativeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NativeOp>")
    }
}

impl Value {
    fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("Value is NOT a number!!"),
        }
    }

    fn as_sym(&self) -> &str {
        if let Self::Sym(sym) = self {
            sym
        } else {
            panic!("value is not a symbol")
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Num(i) => i.to_string(),
            Self::Op(ref s) | Self::Sym(ref s) => s.clone(),
            Self::Block(_) => "<Block>".to_string(),
            Self::Native(_) => "<Native>".to_string(),
        }
    }

    fn to_block(self) -> Vec<Value> {
        match self {
            Self::Block(val) => val,
            _ => panic!("Value is not a BLOCK!!!"),
        }
    }
}

fn puts(vm: &mut Vm) {
    let value = vm.stack.pop().unwrap();
    println!("{}", value.to_string());
}

// fn add(stack: &mut Vec<Value>) {
//     let rhs = stack.pop().unwrap().as_num();
//     let lhs = stack.pop().unwrap().as_num();
//     stack.push(Value::Num(lhs + rhs));
// }

// example 1
// input: 3 1 + { 3 1 + }
// stack: [Num(4), Block([Num(3), Num(1), Op("+")])]
//
// example 2
// input: 3 1 + { { 2 + 1 } 3 + }
// stack: [Num(4), Block([Block([Num(2), Op("+"), Num(1)]), Num(3), Op("+")])]
fn sec2p5() {
    if let Some(f) = std::env::args()
        .nth(1)
        .and_then(|f| std::fs::File::open(f).ok())
    {
        // あえてバッファリングを行わないらしい。
        parse_batch(BufReader::new(f));
    } else {
        parse_interactive();
    }
    // // let mut vm = Vm::new();
    // for line in std::io::stdin().lines().flatten() {
    //     parse(&line);
    //     // parse(&line, &mut vm);
    //     // parse(String::from(line));
    // }
}

fn parse_batch(source: impl BufRead) {
    let mut vm = Vm::new();
    for line in source.lines().flatten() {
        if is_commented_line(&line) {
            continue;
        }

        for word in line.split(" ") {
            parse_word(word, &mut vm);
        }
    }
}

fn parse_interactive() {
    let mut vm = Vm::new();
    for line in std::io::stdin().lines().flatten() {
        if is_commented_line(&line) {
            continue;
        }

        for word in line.split(" ") {
            parse_word(word, &mut vm);
        }
        println!("stack: {:?}", vm.stack);
        println!("vars: {:?}", vm.vars);
    }
}

fn parse_word(word: &str, vm: &mut Vm) {
    if word.is_empty() {
        return;
    }
    if word == "{" {
        vm.blocks.push(vec![])
    } else if word == "}" {
        let top_block = vm.blocks.pop().expect("Block stack underrun!");
        eval(Value::Block(top_block), vm);
    } else {
        let code = if let Ok(num) = word.parse::<i32>() {
            Value::Num(num)
        } else if word.starts_with("/") {
            Value::Sym(word[1..].to_string())
        } else {
            Value::Op(word.to_string())
        };
        eval(code, vm);
    }
}

// // https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html
// // 究極的にライフタイム記法は、関数のいろんな引数と戻り値のライフタイムを接続することに関するものです。
// //
// // fn parse<'vm, 'a>(line: &'a str, vm: &'vm Vm<'a>) -> &'vm [Value<'a>] {
// fn parse<'a>(line: &'a str) -> Vec<Value> {
//     // fn parse(line: &str) -> Vec<Value> {
//     // fn parse<'b>(line: &'b str) {
//     // fn parse(line: &'static str) {
//     // fn parse(line: String) {
//     // let mut stack = vec![];

//     let mut vm: Vm = Vm::new();
//     let mut words: Vec<_> = line.split(" ").collect();

//     while let Some((&word, mut rest)) = words.split_first() {
//         if word == "{" {
//             let value;
//             (value, rest) = parse_block(rest);
//             vm.stack.push(value);
//             // println!("stack: {vm.stack}");
//             // } else if let Ok(parsed) = word.parse::<i32>() {
//             //     stack.push(Value::Num(parsed))
//             // } else {
//             //     match word {
//             //         "+" => add(&mut stack),
//             //         _ => panic!("{word:?} could not be parsed!"),
//             //     }
//             // }
//         } else {
//             let code = if let Ok(num) = word.parse::<i32>() {
//                 Value::Num(num)
//             } else if word.starts_with("/") {
//                 // 今回はこれを変数のはじまり言葉とする！
//                 Value::Sym(word[1..].to_string())
//             } else {
//                 Value::Op(word.to_string())
//             };

//             eval(code, &mut vm);
//         }

//         words = rest.to_vec();
//     }

//     println!("vm.stack: {vm:?}");
//     println!("vm.stack: {:?}", vm.stack);

//     vm.stack
// }

// fn parse_block<'src, 'a>(input: &'a [&'src str]) -> (Value, &'a [&'src str]) {
//     let mut tokens = vec![];
//     let mut words = input;

//     while let Some((&word, mut rest)) = words.split_first() {
//         if word.is_empty() {
//             break;
//         }

//         if word == "{" {
//             let value;
//             (value, rest) = parse_block(rest);
//             tokens.push(value)
//         } else if word == "}" {
//             return (Value::Block(tokens), rest);
//         } else if let Ok(value) = word.parse::<i32>() {
//             tokens.push(Value::Num(value))
//         } else {
//             tokens.push(Value::Op(word))
//         }

//         words = rest;
//     }

//     (Value::Block(tokens), words)
// }

#[cfg(test)]
mod test {
    use super::Value::*;
    // #[test]
    // fn test_group() {
    //     assert_eq!(
    //         parse("1 2 + { 3 4 }"),
    //         vec![Num(3), Block(vec![Num(3), Num(4)])],
    //     )
    // }

    // #[test]
    // fn test_if_false() {
    //     assert_eq!(parse("{ 1 -1 + } { 100 } { -100 } if"), vec![Num(-100)],);
    // }

    // #[test]
    // fn test_if_true() {
    //     assert_eq!(parse("{ 1 1 + } { 100 } { -100 } if"), vec![Num(100)],);
    // }

    // fn parse(input: &str) -> Vec<Value> {
    //     let mut vm = Vm::new();
    //     vm.parse_batch(Cursor::new(input));
    //     vm.eval_all();
    //     vm.get_stack().to_vec()
    // }

    // #[test]
    // fn test_group() {
    //     assert_eq!(
    //         parse_block("1 2 + { 3 4 }"),
    //         vec![Num(3), Block(vec![Num(3), Num(4)])]
    //     )
    // }
}

// fn sec2p4() {
//     for line in std::io::stdin().lines() {
//         let mut stack = vec![];
//         if let Ok(line) = line {
//             let words: Vec<_> = line.split(" ").collect();

//             for word in words {
//                 if let Ok(parsed) = word.parse::<i32>() {
//                     stack.push(parsed);
//                 } else {
//                     match word {
//                         "+" => add(&mut stack),
//                         "-" => sub(&mut stack),
//                         "*" => mul(&mut stack),
//                         "/" => div(&mut stack),
//                         _ => panic!("{word:?} could not be parsed"),
//                     }
//                 }
//             }
//             println!("stack: {stack:?}");
//         }
//     }
// }

// fn sec2p3() {
//     // 42 44 + とか入力させる。
//     for line in std::io::stdin().lines() {
//         if let Ok(line) = line {
//             let words: Vec<_> = line.split(" ").collect();
//             println!("Line: {words:?}");
//         }
//     }
// }

// fn sec2p2() {
//     let mut stack = vec![];

//     stack.push(46);
//     stack.push(2);

//     add(&mut stack);

//     stack.push(22);

//     add(&mut stack);

//     println!("stack: {stack:?}");
// }

// fn add(stack: &mut Vec<i32>) {
//     let rhs = stack.pop().unwrap();
//     let lhs = stack.pop().unwrap();
//     stack.push(lhs + rhs);
// }

// fn sub(stack: &mut Vec<i32>) {
//     let rhs = stack.pop().unwrap();
//     let lhs = stack.pop().unwrap();
//     stack.push(lhs - rhs);
// }

// fn mul(stack: &mut Vec<i32>) {
//     let rhs = stack.pop().unwrap();
//     let lhs = stack.pop().unwrap();
//     stack.push(lhs * rhs);
// }

// fn div(stack: &mut Vec<i32>) {
//     let rhs = stack.pop().unwrap();
//     let lhs = stack.pop().unwrap();
//     stack.push(lhs / rhs);
// }
