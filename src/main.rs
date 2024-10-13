use std::io;

const DELIMETER: &str = "====================";


fn main() {
    let mut app: bool = true;
    while app {
        options_display();
        println!("Enter Selection:");
        let input = get_input().trim().parse::<i32>().unwrap_or(-1);
        app = main_menu_options(input);
    }
    
}

// This is used to display the options to the user
fn options_display() {
    println!("\n{}", DELIMETER);
    println!("Easters Accounting");
    println!("{}\n", DELIMETER);
    println!("1: Bills");
    println!("2: Income");
    println!("3: Ledger");
    println!("4: Exit\n");
}

// This takes a integer input from the user and redirects to the appropriate view
fn main_menu_options(option: i32) -> bool {
    match  option {
        1 => {
            println!("You selected Bills.");
            bill_main_display();
            true
        },
        2 => {
            println!("You selected Income.");
            income_display();
            true
        },
        3 => {
            println!("You selected Ledger.");
            ledger_display();
            true
        },
        4 => {
            println!("Exiting...");
            false
        },
        _ => {
            println!("Invalid selection try again.");
            true
        },
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

// This is used to display the menu for adding, editing or viewing bills.
fn bill_main_display() {
    println!("\n{}", DELIMETER);
    println!("Billing Information");
    println!("\n{}", DELIMETER);
    println!("\tMenu\n");
    println!("1: Add New Bill");
    println!("2: View Current Bills");
    println!("3: Edit Bill");
    println!("4: Return");

    let bill_input = get_input().trim().parse::<i32>().unwrap_or(-1);
    match bill_input {
        1 => {
            println!("Adding new bill...");
        },
        2 => {
            println!("Viewing Current Bills");
        },
        3 => {
            println!("Editing Bill");
        },
        4 => {
            println!("Returing to main menu.");
            return;
        },
        _ => println!("Invalid Selection"),
    }
}

// This is used to display the menu for adding, editing or viewing income sources.
fn income_display() {
    println!("\n{}", DELIMETER);
    println!("Income Information");
    println!("\n{}", DELIMETER);
    println!("\tMenu\n");
    println!("1: Add New Income");
    println!("2: View Current Incomes");
    println!("3: Edit Income");
    println!("4: Return");

    let income_input = get_input().trim().parse::<i32>().unwrap_or(-1);
    match income_input {
        1 => {
            println!("Adding new income...");
        },
        2 => {
            println!("Viewing Current incomes");
        },
        3 => {
            println!("Editing income");
        },
        4 => {
            println!("Returing to main menu.");
            return;
        },
        _ => println!("Invalid Selection"),
    }
}

// This is the menu for viewing ledger information.
fn ledger_display() {
    println!("\n{}", DELIMETER);
    println!("Ledger Information");
    println!("\n{}", DELIMETER);
    println!("\tMenu\n");
    println!("1: View Ledger");
    println!("2: Return");

    let ledger_input = get_input().trim().parse::<i32>().unwrap_or(-1);
    match ledger_input {
        1 => {
            println!("View Ledger...");
        },
        2 => {
            println!("Returing to main menu.");
            return;
        },
        _ => println!("Invalid Selection"),
    }
}