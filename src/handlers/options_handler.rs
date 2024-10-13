use crate::views::app_views;

// This takes a integer input from the user and redirects to the appropriate view
pub fn main_menu_options(option: i8) -> bool {
    match  option {
        1 => {
            println!("You selected Bills.");
            app_views::bill_main_display();
            true
        },
        2 => {
            println!("You selected Income.");
            app_views::income_display();
            true
        },
        3 => {
            println!("You selected Ledger.");
            app_views::ledger_display();
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



// This handles the users input in the bill menu
pub fn bill_menu_options(option: i8) -> bool {
    match option {
        1 => {
            println!("Adding new bill...");
            true
        },
        2 => {
            println!("Viewing Current Bills");
            true
        },
        3 => {
            println!("Editing Bill");
            true
        },
        4 => {
            println!("Returning to main menu.");
            false
        },
        _ => {
            println!("Invalid Entry");
            app_views::bill_main_display();
            true
        },
    }
}

// Handles user input in the income view
pub fn income_menu_options(option: i8) -> bool {
    match option {
        1 => {
            println!("Adding New Income");
            true
        },
        2 => {
            println!("Viewing Current Incomes");
            true
        },
        3 => {
            println!("Editing Income");
            true
        },
        4 => {
            println!("Returning to Main Menu");
            false
        },
        _ => {
            println!("Error, Invalid Selection");
            app_views::income_display();
            true
        },
    }
}



// Handles user inputs in the ledger view
pub fn ledger_menu_options(option: i8) -> bool {
    match option {
        1 => {
            println!("View Ledger");
            true
        },
        2 => {
            println!("Returning to Main Menu");
            false
        }
        _ => {
            println!("Error, Invalid Selection");
            app_views::ledger_display();
            true
        },
    }
}