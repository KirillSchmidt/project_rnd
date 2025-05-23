// pub fn display_dice_result<I: PartialOrd + ToString>(result_to_show: I) {
//     let int_length = result_to_show.to_string().len();
//     let max_bound = int_length + 4; // length + 2 spaces + 2 chars of |
//     for row in 0..=max_bound {
//         for column in 0..=max_bound {
//             if row == 0 || row == max_bound { // the top or the bottom
//                 print!("—")
//             } else if column == 0 || column == max_bound { // left-most or right-most
//                 print!("|")
//             } else if column == max_bound + 2 && row == { // place for the result to show
//                 todo!()
//             } else {
//                 print!(" ");
//             }
//         }
//         println!();
//     }
//     println!();
// }
//
// /*
//
// ———————————
// |
// |   3     |
// |
// ————————————
//  */
