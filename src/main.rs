mod menu_choice;

use std::io::Write;
use std::process::exit;
use rand::Rng;
use menu_choice::MenuChoice;

fn generate_dice_result(min: i8, max: i8) -> i8 {
    let random_number = rand::rng().random_range(min..=max);
    return random_number;
}

fn main() {
    menu_choice::print_all_menu_options();

    'main_loop: loop {
        print!("Enter the menu choice: ");
        std::io::stdout().flush().expect("Can't flush the stdout");
        // TODO: implement the repetition of input in the `get_menu_option`
        let user_option = menu_choice::get_menu_option();
        match user_option {
            MenuChoice::D6 => {println!("{}", generate_dice_result(1, 6))}
            MenuChoice::RandomDiceUserInputRange => {println!("Can't do that yet")},
            MenuChoice::Exit => {
                break 'main_loop;
            }
        }
    }

    println!("Exiting...");
    exit(0);
}
