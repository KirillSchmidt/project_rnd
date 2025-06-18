#![allow(dead_code)]
use artem::config::Config;
use artem::ConfigBuilder;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};

fn create_config() -> Config {
    let mut builder = ConfigBuilder::new();
    builder.target_size(NonZeroU32::new(50).unwrap());
    builder.scale(0.35_f32);
    return builder.build();
}

pub fn get_ascii_of_image(filepath: Box<Path>) -> Result<String, String> {
    let fp_os_str = filepath.as_os_str();
    let required_extension = "jpeg"; // TODO: made extensions be a list, not a single value
    return if !filepath.exists() {
        Err(format!("File {:#?} does not exist", fp_os_str))
    } else if !filepath.is_file() {
        Err(format!("{:#?} is not a file", fp_os_str))
    } else if let Some(ext) = filepath.extension() {
        if ext != required_extension {
            Err(format!(
                "{:#?} should have had extension {:#?}, not {:#?}",
                fp_os_str, required_extension, ext
            ))
        } else {
            Err(format!("Can't parse extension in {:#?}", fp_os_str))
        }
    } else {
        let img = image::open(filepath).expect("Failed to open an image");
        Ok(artem::convert(img, &create_config()))
    };
}

pub fn get_ascii_of_standard_dice(side: u8) -> Result<String, String> {
    let path = PathBuf::from(format!("resources/img/face-{side}.jpeg"));
    return get_ascii_of_image(path.into());
}
