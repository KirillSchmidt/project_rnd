use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuChoice {
    CoinFlip,
    D6,
    RandomDiceUserInputRange,
    Exit,
}

impl fmt::Display for MenuChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            MenuChoice::D6 => "Throw a D6 (regular dice)",
            MenuChoice::CoinFlip => "Flip a coin",
            MenuChoice::RandomDiceUserInputRange => "Throw a custom dice",

            MenuChoice::Exit => "Exit",
        };
        return write!(f, "{}", s);
    }
}

pub const LEN_OF_ALL_OPTIONS: usize = 4;
pub const ALL_OPTIONS: [MenuChoice; LEN_OF_ALL_OPTIONS] = [
    MenuChoice::CoinFlip,
    MenuChoice::D6,
    MenuChoice::RandomDiceUserInputRange,
    MenuChoice::Exit,
];

pub fn get_all_menu_strings() -> Vec<String> {
    let mut all_options: Vec<String> = vec![];
    all_options.reserve_exact(LEN_OF_ALL_OPTIONS);
    let opt_iter = ALL_OPTIONS.iter();
    for opt in opt_iter {
        all_options.push(opt.to_string());
    }
    return all_options;
}

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
        let i_option =
            index_to_choice(i).expect("Invalid index for MenuChoice. Potential out-of-bounds");
        println!("{}", i_option);
    }
    println!();
}

// pub fn get_menu_option() -> MenuChoice {
//     let user_int = crate::get_valid_int::<u8>(1, choice_to_index(MenuChoice::Exit).unwrap() + 1);
//
//     match index_to_choice((user_int - 1) as usize) {
//         Some(choice) => return choice,
//         None => {
//             panic!("Did not expect to get invalid range of index")
//         }
//     }
// }
