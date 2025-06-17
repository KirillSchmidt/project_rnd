
// use std::str::FromStr;
//
// fn print_and_flush(str_to_print: &str) {
//     print!("{}", str_to_print);
//     std::io::stdout().flush().unwrap(); // NOTE: could handle the error case, maybe
// }
//
// fn print_flush_and_clear(str_to_print: &str, mut string_to_clear: String) -> String {
//     print_and_flush(str_to_print);
//     string_to_clear.clear();
//     return string_to_clear;
// }
//
// fn get_valid_int<I: FromStr + PartialOrd>(min: I, max: I) -> I
// where
//     <I as FromStr>::Err: std::fmt::Display,
// {
//     let mut input_line = String::new();
//     let mut user_int: I;
//
//     'until_valid: loop {
//         std::io::stdin()
//             .read_line(&mut input_line)
//             .expect("Failed to read the input line");
//         let check_int = input_line.trim().parse::<I>();
//         match check_int {
//             Ok(correct_int) => user_int = correct_int,
//             Err(e) => {
//                 input_line = print_flush_and_clear(&format!("{e}. Try again: "), input_line);
//                 continue 'until_valid;
//             }
//         }
//
//         // up to here, the value is guaranteed to be an integer (type I)
//         if user_int < min {
//             input_line = print_flush_and_clear("The value is too small. Try again: ", input_line);
//             continue 'until_valid;
//         } else if user_int > max {
//             input_line = print_flush_and_clear("The value is too large. Try again: ", input_line);
//             continue 'until_valid;
//         } else {
//             return user_int;
//         }
//     }
// }
