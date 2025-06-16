use rand::Rng;

pub fn generate_dice_result(min: i8, max: i8) -> i8 {
    // TODO: do checks for max > min || max >= min
    let random_number = rand::rng().random_range(min..=max);
    return random_number;
}

pub fn flip_coin() -> String {
    return if generate_dice_result(0, 1) == 0 {
        String::from("Heads")
    } else {
        String::from("Tails")
    };
}

pub fn standard_dice() -> u8 {
    return generate_dice_result(1, 6) as u8;
}
