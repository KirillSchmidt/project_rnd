use rand::Rng;

pub fn generate_dice_result(min: i8, max: i8) -> i8 {
    // TODO: do checks for max > min || max >= min
    let random_number = rand::rng().random_range(min..=max);
    return random_number;
}
