mod slist;

fn main() {
    let mut list = slist::List::new();

    list.push(1);
    list.push(2);
    list.push(3);
    list.unshift(4);
    println!("list[0] = {}", list.get(0).unwrap());
    println!("list[1] = {}", list.get(1).unwrap());
    println!("list[2] = {}", list.get(2).unwrap());
    println!("list[3] = {}", list.get(3).unwrap());
}
