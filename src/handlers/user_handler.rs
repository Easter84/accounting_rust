use std::io;

// This is used to get the input from the user.
pub fn get_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read");

    return user_input;
}