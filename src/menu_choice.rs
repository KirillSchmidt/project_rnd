#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuChoice {
    CoinFlip,
    D6,
    RandomDiceUserInputRange,
    Exit,
}

const LEN_OF_ALL_OPTIONS: usize = 4;
const ALL_OPTIONS: [MenuChoice; LEN_OF_ALL_OPTIONS] = [
    MenuChoice::CoinFlip,
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
    for (i, i_choice) in ALL_OPTIONS.iter().enumerate().take(LEN_OF_ALL_OPTIONS) {
        if i_choice == &choice {
            return Some(i as u8);
        }
    }
    return None;
}

pub fn print_all_menu_options() {
    for i in 0..ALL_OPTIONS.len() {
        print!("{}. ", i + 1);
        let option =
            index_to_choice(i).expect("Invalid index for MenuChoice. Potential out-of-bounds");
        match option {
            MenuChoice::D6 => print!("Throw a D6 (regular dice)"),
            MenuChoice::CoinFlip => print!("Flip a coin"),
            MenuChoice::RandomDiceUserInputRange => print!("Throw a custom dice"),

            MenuChoice::Exit => print!("Exit"),
        }
        println!();
    }
    println!();
}

pub fn get_menu_option() -> MenuChoice {
    let user_int = crate::get_valid_int::<u8>(1, choice_to_index(MenuChoice::Exit).unwrap() + 1);

    match index_to_choice((user_int - 1) as usize) {
        Some(choice) => return choice,
        None => {
            panic!("Did not expect to get invalid range of index")
        }
    }
}
