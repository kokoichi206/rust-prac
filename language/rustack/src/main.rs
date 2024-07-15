fn main() {
    // sec2p2();
    // sec2p3();
    // sec2p4();
    sec2p5();
}

fn eval<'src>(code: Value<'src>, stack: &mut Vec<Value<'src>>) {
    // fn eval(code: Value, stack: &mut Vec<Value>) {
    match code {
        Value::Op(op) => match op {
            "+" => add(stack),
            "if" => op_if(stack),
            _ => panic!("{op:?} could not be parsed!!!"),
        },
        _ => stack.push(code.clone()),
    }
}

fn op_if(stack: &mut Vec<Value>) {
    let false_branch = stack.pop().unwrap().to_block();
    let true_branch = stack.pop().unwrap().to_block();
    let cond = stack.pop().unwrap().to_block();

    for code in cond {
        eval(code, stack);
    }

    let cond_result = stack.pop().unwrap().as_num();

    if cond_result != 0 {
        for cond in true_branch {
            eval(cond, stack);
        }
    } else {
        for code in false_branch {
            eval(code, stack);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)] // トレイトの継承
enum Value<'src> {
    // スタック上に push された数値。
    Num(i32),
    // 演算子。
    Op(&'src str),
    // ネストされたブロック（？）
    // { } とかで囲まれたところを表す。
    Block(Vec<Value<'src>>),
}

impl<'src> Value<'src> {
    fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("Value is NOT a number!!"),
        }
    }
}

impl<'src> Value<'src> {
    fn to_block(self) -> Vec<Value<'src>> {
        match self {
            Self::Block(val) => val,
            _ => panic!("Value is not a BLOCK!!!"),
        }
    }
}

fn add(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs + rhs));
}

// example 1
// input: 3 1 + { 3 1 + }
// stack: [Num(4), Block([Num(3), Num(1), Op("+")])]
//
// example 2
// input: 3 1 + { { 2 + 1 } 3 + }
// stack: [Num(4), Block([Block([Num(2), Op("+"), Num(1)]), Num(3), Op("+")])]
fn sec2p5() {
    for line in std::io::stdin().lines().flatten() {
        parse(&line);
        // parse(String::from(line));
    }
}

// https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html
// 究極的にライフタイム記法は、関数のいろんな引数と戻り値のライフタイムを接続することに関するものです。
//
// fn parse<'a>(line: &'a str) {
fn parse(line: &str) -> Vec<Value> {
    // fn parse<'b>(line: &'b str) {
    // fn parse(line: &'static str) {
    // fn parse(line: String) {
    let mut stack = vec![];
    let mut words: Vec<_> = line.split(" ").collect();

    while let Some((&word, mut rest)) = words.split_first() {
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            stack.push(value);
            println!("stack: {stack:?}");
        // } else if let Ok(parsed) = word.parse::<i32>() {
        //     stack.push(Value::Num(parsed))
        // } else {
        //     match word {
        //         "+" => add(&mut stack),
        //         _ => panic!("{word:?} could not be parsed!"),
        //     }
        // }
        } else {
            let code = if let Ok(num) = word.parse::<i32>() {
                Value::Num(num)
            } else {
                Value::Op(word)
            };

            eval(code, &mut stack);
        }

        words = rest.to_vec();
    }

    println!("stack: {stack:?}");

    stack
}

fn parse_block<'src, 'a>(input: &'a [&'src str]) -> (Value<'src>, &'a [&'src str]) {
    let mut tokens = vec![];
    let mut words = input;

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }

        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            tokens.push(value)
        } else if word == "}" {
            return (Value::Block(tokens), rest);
        } else if let Ok(value) = word.parse::<i32>() {
            tokens.push(Value::Num(value))
        } else {
            tokens.push(Value::Op(word))
        }

        words = rest;
    }

    (Value::Block(tokens), words)
}

#[cfg(test)]
mod test {
    use super::{parse, Value::*};
    #[test]
    fn test_group() {
        assert_eq!(
            parse("1 2 + { 3 4 }"),
            vec![Num(3), Block(vec![Num(3), Num(4)])],
        )
    }

    #[test]
    fn test_if_false() {
        assert_eq!(
            parse("{ 1 -1 + } { 100 } { -100 } if"),
            vec![Num(-100)],
        );
    }

    #[test]
    fn test_if_true() {
        assert_eq!(
            parse("{ 1 1 + } { 100 } { -100 } if"),
            vec![Num(100)],
        );
    }

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
