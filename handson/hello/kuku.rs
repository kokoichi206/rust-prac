fn main() {
    for i in 1..=9 {
        let s: String = (1..10)
            .map(|j| format!("{:3}", i * j))
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", s);

        // for j in 1..10 {
        //     // print!("{:3},", i * j);
        // }
        // println!("");
    }
}
