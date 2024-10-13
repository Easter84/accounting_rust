mod views;
mod handlers;


fn main() {
    let mut app: bool = true;
    while app {
        views::app_views::main_options_display();
        println!("Enter Selection:");
        let input = handlers::user_handler::get_input().trim().parse::<i8>().unwrap_or(-1);
        app = handlers::options_handler::main_menu_options(input);
    }    
}