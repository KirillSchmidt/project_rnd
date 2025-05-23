mod dice_generation;
mod ascii_graphics;
mod menu_choice;

use menu_choice::MenuChoice;
use std::io::Write;
use std::process::exit;
use std::str::FromStr;

fn main() {
    menu_choice::print_all_menu_options();

    'main_loop: loop {
        print!("Enter the menu choice: ");
        std::io::stdout().flush().expect("Can't flush the stdout");
        let user_option = menu_choice::get_menu_option();
        match user_option {
            MenuChoice::D6 => {
                let result = dice_generation::generate_dice_result(1, 6);
                ascii_graphics::display_standard_dice(result as u8);
                // println!("The result: {}", dice_generation::generate_dice_result(1, 6))
            },
            MenuChoice::RandomDiceUserInputRange => {
                println!("Enter the lowest value (included): ");
                let lowest = get_valid_int::<i8>(i8::MIN, i8::MAX);
                println!("Enter the highest value (included): ");
                let highest = get_valid_int::<i8>(lowest + 1, i8::MAX);
                println!(
                    "The result: {}",
                    dice_generation::generate_dice_result(lowest, highest)
                )
            },
            MenuChoice::Exit => {
                break 'main_loop;
            },
        }
    }

    println!("Exiting...");
    exit(0);
}

fn print_and_flush(str_to_print: &str) {
    print!("{}", str_to_print);
    std::io::stdout().flush().unwrap(); // NOTE: could handle the error case, maybe
}

fn print_flush_and_clear(str_to_print: &str, mut string_to_clear: String) -> String {
    print_and_flush(str_to_print);
    string_to_clear.clear();
    return string_to_clear;
}

fn get_valid_int<I: FromStr + PartialOrd>(min: I, max: I) -> I
where
    <I as FromStr>::Err: std::fmt::Display,
{
    let mut input_line = String::new();
    let mut user_int;

    'until_valid: loop {
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read the input line");
        let check_int = input_line.trim().parse::<I>();
        match check_int {
            Ok(correct_int) => user_int = correct_int,
            Err(e) => {
                input_line = print_flush_and_clear(&format!("{e}. Try again: "), input_line);
                continue 'until_valid;
            }
        }

        // up to here, the value is guaranteed to be an integer (type I)
        if user_int < min {
            input_line = print_flush_and_clear("The value is too small. Try again: ", input_line);
            continue 'until_valid;
        } else if user_int > max {
            input_line = print_flush_and_clear("The value is too large. Try again: ", input_line);
            continue 'until_valid;
        } else {
            return user_int;
        }
    }
}
