use crate::handlers::user_handler;
use crate::handlers::options_handler;

const DELIMETER: &str = "====================";

// This is used to display the options to the user
pub fn main_options_display() {
    println!("\n{}", DELIMETER);
    println!("Easters Accounting");
    println!("{}\n", DELIMETER);
    println!("1: Bills");
    println!("2: Income");
    println!("3: Ledger");
    println!("4: Exit\n");
}

// This is used to display the menu for adding, editing or viewing bills.
pub fn bill_main_display() {
    println!("\n{}", DELIMETER);
    println!("Billing Information");
    println!("\n{}", DELIMETER);
    println!("\tMenu\n");
    println!("1: Add New Bill");
    println!("2: View Current Bills");
    println!("3: Edit Bill");
    println!("4: Return");

    
    let bill_input = user_handler::get_input().trim().parse::<i8>().unwrap_or(-1);
    options_handler::bill_menu_options(bill_input);
}

// This is used to display the menu for adding, editing or viewing income sources.
pub fn income_display() {
    println!("\n{}", DELIMETER);
    println!("Income Information");
    println!("\n{}", DELIMETER);
    println!("\tMenu\n");
    println!("1: Add New Income");
    println!("2: View Current Incomes");
    println!("3: Edit Income");
    println!("4: Return");

    let income_input = user_handler::get_input().trim().parse::<i8>().unwrap_or(-1);
    options_handler::income_menu_options(income_input);
}

// This is the menu for viewing ledger information.
pub fn ledger_display() {
    println!("\n{}", DELIMETER);
    println!("Ledger Information");
    println!("\n{}", DELIMETER);
    println!("\tMenu\n");
    println!("1: View Ledger");
    println!("2: Return");

    let ledger_input = user_handler::get_input().trim().parse::<i8>().unwrap_or(-1);
    options_handler::ledger_menu_options(ledger_input);
}