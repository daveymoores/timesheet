use std::io;

#[derive(Debug)]
struct User<'a> {
    first_name: &'a str,
    surname: &'a str,
}

// This function takes a prompt referenced string which is borrowed
// as it is declared outside of the function. It points to a specific utf-8 sequence and is therefore stack
// allocated. The input however is a vector of bytes that is heap allocated. I borrow a mutable reference here.
fn get_name<'a>(prompt: &'a str, input: &'a mut String) -> &'a str {
    println!("Please enter a {}:", prompt);

    io::stdin()
        .read_line(input)
        .expect("Failed to read line");

    input.trim()
}

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    let first_name = get_name("first name", &mut x);
    let surname = get_name("surname", &mut y);

    let user = User {
        first_name,
        surname
    };

    println!("User is called: {} {}", user.first_name, user.surname);
    println!("{:#?}", user);
}