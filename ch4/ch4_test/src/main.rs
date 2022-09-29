fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || item == b',' {
            return &s[..i];
        }
    } 

    &s[..]
}

fn main() {
    let my_str = String::from("hello, world!");

    // first_word works on slices of String's
    let word = first_word(&my_str[..]);

    println!("{}", word);

    let my_str_literal = "hello, world";

    // first_word works on slices of string literals 
    let word = first_word(&my_str_literal[..]);

    println!("{}", word);


    // Because string literals are string slices already, this works too, without the slice syntax
    let word = first_word(my_str_literal);

    println!("{}", word);
}