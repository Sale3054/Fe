fn main() {
    let my_string = String::from("Hello World, foo");

    let num: usize = first_word(&my_string);
    println!("First word: {}", &my_string[..num]);
    //              01234
    let test_str = "abcde";
    println!("{}", &test_str[0..3]);

    let len_of_slice: usize = 3;
    println!("{}", &test_str[0..len_of_slice])
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
