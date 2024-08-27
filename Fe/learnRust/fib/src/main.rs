use std::io;
fn main() {
    let mut usr_input = String::new();
    println!("Let's calculate a fibonacci number! \n What do you want to calculate? ");

    loop {
        usr_input.clear();

        io::stdin()
            .read_line(&mut usr_input)
            .expect("you fucked up");

        usr_input = usr_input.trim().to_string();
        println!("You typed: '{}'", usr_input);
        if usr_input.contains("break") {
            break;
        }
        //convert the number to an int
        let num: u64 = usr_input.parse::<u64>().unwrap();
        println!("Result: {}", get_fib(num));
    }
}

fn get_fib(nth: u64) -> u64 {
    if nth == 0 {
        0
    } else if nth == 1 {
        1
    } else {
        get_fib(nth - 1) + get_fib(nth - 2)
    }
}
