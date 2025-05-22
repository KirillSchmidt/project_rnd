use std::io::Write;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuChoice {
    D6,
    RandomDiceUserInputRange,
    Exit,
}

const LEN_OF_ALL_OPTIONS: usize = 3;
const ALL_OPTIONS: [MenuChoice; LEN_OF_ALL_OPTIONS] = [
    MenuChoice::D6,
    MenuChoice::RandomDiceUserInputRange,
    MenuChoice::Exit,
];

pub fn index_to_choice(index: usize) -> Option<MenuChoice> {
    return match index {
        0..LEN_OF_ALL_OPTIONS => Some(ALL_OPTIONS[index]),
        _ => None,
    };
}

pub fn choice_to_index(choice: MenuChoice) -> Option<u8> {
    for i in 0..LEN_OF_ALL_OPTIONS {
        if ALL_OPTIONS[i] == choice {
            return Some(i as u8);
        }
    }
    return None;
}

fn print_flush_and_clear(str_to_print: &str, mut string_to_clear: String) -> String {
    print!("{}", str_to_print);
    std::io::stdout().flush().unwrap(); // NOTE: could handle the error case, maybe
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
                input_line = print_flush_and_clear(&format!("{e}. Try again: "), input_line); // "invalid digit found in string"
                continue 'until_valid;
            }
        }
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

pub fn print_all_menu_options() {
    for i in 0..ALL_OPTIONS.len() {
        print!("{}. ", i + 1);
        let option =
            index_to_choice(i).expect("Invalid index for MenuChoice. Potential out-of-bounds");
        match option {
            MenuChoice::D6 => print!("Throw a D6 (regular dice)"),
            MenuChoice::RandomDiceUserInputRange => print!("Throw a custom dice"),
            MenuChoice::Exit => print!("Exit"),
        }
        println!();
    }
    println!();
}

pub fn get_menu_option() -> MenuChoice {
    let user_int = get_valid_int::<u8>(1, choice_to_index(MenuChoice::Exit).unwrap() + 1);

    match index_to_choice((user_int - 1) as usize) {
        Some(choice) => return choice,
        None => {
            todo!("Did not expect to get invalid range of index")
        }
    }
}
