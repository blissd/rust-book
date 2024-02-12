fn main() {
    let s = String::from("Hello, world!");
    let i = first_word(&s);
    println!("{} first word ends at {}", s, i);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()}
