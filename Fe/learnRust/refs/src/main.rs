fn main() {
    let h: String = String::from("Hello world");

    println!("{}", h);

    let s: &str = &h;
    let z: String = h.clone();

    no_steal(s);
    no_steal(h);
    println!("Here are the values as I understand them; z: {z}, y: {y}, h: {h}");
}

fn stealsValue(s: String) -> String {
    println!("Invoking this function stole {}!", s);
    // return s so that we can take it back
    s
}

fn no_steal(s: &str) {
    println!("{s}");
}

fn borrowsValue(s: &str) {
    println!("Invoking this function borrowed {}!", s);
}
