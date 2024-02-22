fn main() {
    println!("apple = {}", to_pig_latin("apple"));
    println!("Apple = {}", to_pig_latin("Apple"));
    println!("yellow = {}", to_pig_latin("yellow"));
    println!("Yellow = {}", to_pig_latin("Yellow"));
}

fn to_pig_latin(s: &str) -> String {
    match s.chars().next() {
        None => String::from(s),
        Some(c) if is_vowel(c) => String::from(s.to_string() + "-hay"),
        Some(c) => {
            let mut pig_latin = String::new();
            let chars = s[1..].chars();

            for (i, v) in chars.enumerate() {
                if i == 0 && c.is_ascii_uppercase() {
                    pig_latin.push(v.to_ascii_uppercase());

                } else {
                    pig_latin.push(v);
                }
            }

            let suffix = format!("-{c}ay").to_lowercase();
            pig_latin.push_str(&suffix);
            pig_latin
        }
    }
}

fn is_vowel(c: char) -> bool {
    let c = c.to_ascii_lowercase();
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
