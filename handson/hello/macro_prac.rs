macro_rules! echo_num {
    // expr というのは式を表す。
    ($num:expr) => {
        println!("num = {}", $num);
    };
}

#[macro_export]
macro_rules! echo_nums {
    // 可変長引数を受け取る。
    ( $($num:expr),* ) => {
        $(
            print!("{}, ", $num);
        )*
        println!();
    };
}

macro_rules! easy_for {
    // for i = 1 to 10
    (
        for $i:ident = $from:tt to $to:tt
        $block:block
    ) => {{
        for $i in $from..=$to {
            $block
        }
    }};
    // for i = 1 to 10 step 2
    (
        for $i:ident = $from:tt to $to:tt step $step:tt
        $block:block
    ) => {{
        let mut $i = $from;
        loop {
            if $i > $to {
                break;
            }
            $block
            $i += $step;
        }
    }};
}

fn main() {
    // (), [], {} はどれでもいい。
    echo_num!(5);
    echo_num![10];
    echo_num! {15}

    echo_nums!(1, 2, 3);

    let mut total = 0;
    easy_for! {
        for i = 1 to 10 step 2 {
            total += i;
            println!("i = {}, total = {}", i, total);
        }
    }
}
