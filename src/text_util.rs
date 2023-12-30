use std::cmp::min;
use std::io;
use std::io::Write;

pub fn get_input(prompt: impl Into<String>) -> String {
    let str_prompt: String = prompt.into();
    print!("{str_prompt}");
    io::stdout().flush().expect("Flushing stdout");

    let mut output = String::new();
    io::stdin().read_line(&mut output).expect("Reading line from stdin");
    return output.trim().to_string();
}

pub fn get_integer(prompt: impl Into<String> + Copy) -> i32 {

    loop {
        match get_input(prompt).parse() {
            Ok(parsed) => {
                return parsed;
            },
            Err(_) => {
                println!("Please enter a positive integer");
            }
        }
    }
}

pub fn get_integer_in_range(min_n: i32, max_n: i32, prompt: impl Into<String> + Copy) -> i32 {
    loop {
        let integer = get_integer(prompt);
        if integer < min_n || integer > max_n {
            println!("Number must be between {min_n} and {max_n} (inclusive)");
        } else {
            return integer;
        }
    }
}