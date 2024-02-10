fn main() {
    let s = String::from("hello");
    take_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}
