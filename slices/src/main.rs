fn main() {
    // String Slice
    let s = String::from("hello world");

    println!("First word = {}", first_word(&s[..]));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
