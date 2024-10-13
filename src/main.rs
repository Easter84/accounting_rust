use std::io;

fn main() {
    options_display();
    println!("Enter Selection:");
    let input = get_input().trim().parse::<i32>().unwrap_or(-1);
    match_options(input);
}

// This is used to display the options to the user
fn options_display() {
    let delimeter = "=".repeat(20);
    println!("\n{}", delimeter);
    println!("Easters Accounting");
    println!("{}\n", delimeter);
    println!("1: Bills");
    println!("2: Income");
    println!("3: Ledger");
    println!("4: Exit\n");
}

// This takes a integer input from the user and redirects to the appropriate view
fn match_options(option: i32) {
    match  option {
        1 => println!("You selected Bills."),
        2 => println!("You selected Income."),
        3 => println!("You selected Ledger."),
        4 => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => println!("Invalid selection try again."),
    }
}

// This is used to get the input from the user.
fn get_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read");

    return user_input;
}