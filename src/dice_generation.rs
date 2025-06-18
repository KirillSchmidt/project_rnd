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

pub fn custom_dice(min: i8, max: i8) -> Result<i8, String> {
    return if min > max {
        Err("Min should be less than max".into())
    } else if min == max {
        Err(format!(
            "If you make min = max, the result is the value itself ({} in your case)",
            min
        ))
    } else if max == min + 1 {
        Err(format!(
            "You might as well just flip a coin\n(it's {} btw)",
            generate_dice_result(min, max)
        ))
    } else if max == min + 5 {
        Err(format!(
            "You might as well just use a standard dice\n(it's {} btw)",
            generate_dice_result(min, max)
        ))
    } else {
        Ok(generate_dice_result(min, max))
    };
}
