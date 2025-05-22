use std::io::Write;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuChoice {
    D6,
    RandomDiceUserInputRange,
    Exit
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
        _ => None
    }
}

pub fn choice_to_index(choice: MenuChoice) -> Option<u8> {
    for i in 0..LEN_OF_ALL_OPTIONS {
        if ALL_OPTIONS[i] == choice {
            return Some(i as u8)
        }
    }
    return None;
}

pub fn print_all_menu_options() {
    for i in 0..ALL_OPTIONS.len() {
        print!("{}. ", i+1);
        let option = index_to_choice(i).expect("Invalid index for MenuChoice. Potential out-of-bounds");
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
    let mut input_line = String::new();
    'until_valid: loop {
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read the input line");
        let check_int = input_line.trim().parse::<u8>();
        let mut user_int = 0;
        match check_int {
            Ok(correct_int) => user_int = correct_int,
            Err(e) => {
                print!("{e}. Try again: "); // "invalid digit found in string"
                // println!("Error while parsing <{}>: {e}", input_line.trim());
                input_line.clear();
                std::io::stdout().flush().unwrap(); // NOTE: could be put into a function
                continue;
            }
        }

        match index_to_choice((user_int-1) as usize) {
            Some(choice) => return choice,
            None => {
                print!("Invalid menu option. Try again: ");
                input_line.clear();
                std::io::stdout().flush().unwrap(); // NOTE: could be put into a function
            }
        }
    }
}