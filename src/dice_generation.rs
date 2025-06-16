use rand::Rng;

fn generate_dice_result(min: i8, max: i8) -> i8 {
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

pub fn custom_dice(side: i8) -> Result<i8, String> {
    if side <= 0 {
        return Err("The value should be larger than 0".into());
    }
    let result = generate_dice_result(1, side);
    if side > 2 {
        return Ok(result);
    }
    // Let the fun begin!
    let mut funny_error_msg = String::new();
    if side == 1 {
        funny_error_msg.push_str("It doesn't make any sense! It will always be equal to 1");
    } else if side == 2 {
        funny_error_msg.push_str("Just use the 'Flip a coin' option...");
    }
    funny_error_msg += &format!("\n(it's {} btw)", result);
    return Err(funny_error_msg);
}