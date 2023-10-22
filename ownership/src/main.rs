fn main() {
    let mut s = String::from("Hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let word = first_word(&s);
    s.clear(); // The error is here
    println!("{word}");

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

