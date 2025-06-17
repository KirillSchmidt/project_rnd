#![allow(dead_code)]
use artem::config::Config;
use artem::ConfigBuilder;
use std::num::NonZeroU32;

fn create_config() -> Config {
    let mut builder = ConfigBuilder::new();
    builder.target_size(NonZeroU32::new(50).unwrap());
    builder.scale(0.35_f32);
    return builder.build();
}

pub fn display_standard_dice(side: u8) {
    assert!((1..=6).contains(&side));
    let filepath = format!("resources/img/face-{side}.jpeg");
    assert!(std::fs::exists(&filepath).unwrap());
    let img = image::open(filepath).expect("Failed to open an image");
    let ascii_art = artem::convert(img, &create_config());
    println!("{}", ascii_art);
}

// pub fn test_artem() {
//     for i in 1..=6 {
//         display_standard_dice(i);
//     }
// }
