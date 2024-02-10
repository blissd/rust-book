fn main() {
    let s1 = give_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(s: String) -> String {
    s
}
