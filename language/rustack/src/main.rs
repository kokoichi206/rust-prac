fn main() {
    // sec2p2();
    // sec2p3();
    // sec2p4();
    sec2p5();
}

#[derive(Debug, PartialEq, Eq)] // トレイトの継承
enum Value<'src> {
    Num(i32),
    Op(&'src str),
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

fn add(stack: &mut Vec<Value>) {
    let rhs = stack.pop().unwrap().as_num();
    let lhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs + rhs));
}

fn sec2p5() {
    for line in std::io::stdin().lines() {
        let mut stack = vec![];
        if let Ok(line) = line {
            let words: Vec<_> = line.split(" ").collect();

            while let Some((&word, mut rest)) = words.split_first() {
                if word == "{" {
                    let value;
                    (value, rest) = parse_block(rest);
                    stack.push(value);
                } else {
                    // ...
                }
                words = rest;
            }

            // for word in words {
            //     if let Ok(parsed) = word.parse::<i32>() {
            //         stack.push(parsed);
            //     } else {
            //         match word {
            //             "+" => add(&mut stack),
            //             "-" => sub(&mut stack),
            //             "*" => mul(&mut stack),
            //             "/" => div(&mut stack),
            //             _ => panic!("{word:?} could not be parsed"),
            //         }
            //     }
            // }
            println!("stack: {stack:?}");
        }
    }
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
    // use super::{Value::*, *};
    // use std::io::Cursor;

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
