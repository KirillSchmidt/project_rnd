mod dice_generation;
mod menu_choice;

use menu_choice::MenuChoice;
// use rand::Rng;
use std::io::Write;
use std::process::exit;

fn main() {
    menu_choice::print_all_menu_options();

    'main_loop: loop {
        print!("Enter the menu choice: ");
        std::io::stdout().flush().expect("Can't flush the stdout");
        // TODO: implement the repetition of input in the `get_menu_option`
        let user_option = menu_choice::get_menu_option();
        match user_option {
            MenuChoice::D6 => {
                println!("{}", dice_generation::generate_dice_result(1, 6))
            }
            MenuChoice::RandomDiceUserInputRange => {
                println!("Can't do that yet")
            }
            MenuChoice::Exit => {
                break 'main_loop;
            }
        }
    }

    println!("Exiting...");
    exit(0);
}
