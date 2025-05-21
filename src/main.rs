use std::io::Write;
use std::process::exit;
use rand::Rng;


enum MenuOption {
    D6,
    RandomDiceUserInputRange,
    Exit
}


fn generate_dice_result(min: i8, max: i8) -> i8 {
    let random_number = rand::rng().random_range(min..=max);
    return random_number;
}

fn print_all_menu_options() {
    static ALL_OPTIONS: [MenuOption; 3] = [
        MenuOption::D6,
        MenuOption::RandomDiceUserInputRange,
        MenuOption::Exit,
    ];

    for i in 0..ALL_OPTIONS.len() {
        print!("{}. ", i+1);
        let option = &ALL_OPTIONS[i];
        match option {
            MenuOption::D6 => print!("Throw a D6 (regular dice)"),
            MenuOption::Exit => print!("Exit"),
            _ => print!("Not implemented yet")
        }
        println!();
    }
    println!();
}

fn get_menu_option() -> Option<MenuOption> {
    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read the input line");
    let user_int: u8 = input_line.trim().parse().expect("Value should be an integer");

    return match user_int {
        1 => Some(MenuOption::D6),
        2 => Some(MenuOption::RandomDiceUserInputRange),
        _ => None
    }
}

fn main() {
    print_all_menu_options();

    'main_loop: loop {
        print!("Enter the menu choice: ");
        std::io::stdout().flush().expect("Can't flush the stdout");
        // TODO: implement the repetition of input in the `get_menu_option`
        let user_option = get_menu_option().expect("Expected a correct value");
        match user_option {
            MenuOption::D6 => {println!("{}", generate_dice_result(1, 6))}
            MenuOption::RandomDiceUserInputRange => {println!("Can't do that yet")},
            MenuOption::Exit => {
                break 'main_loop;
            }
        }
    }

    println!("Exiting...");
    exit(0);
}
