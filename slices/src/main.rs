fn main() {
    let s = String::from("Hello world!");
    let w = first_word(&s);
    println!("{} first word is {}", s, w);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
