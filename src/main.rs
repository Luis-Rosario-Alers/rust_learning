mod chapter_8;
use crate::chapter_8::{find_median, pig_latin_conversion, _find_mode, MedianResult};
use std::io::{self};
fn main() {
    let integers_list: [i32; 6] = [1, 1, 5, 6, 6, 7];

    println!(
        "The median for this list is {}",
        match find_median(&integers_list) {
            MedianResult::Float(floating_num) => floating_num,
            MedianResult::Int(int_num) => int_num as f32,
        }
    );

    println!("The mode of this list is {}", _find_mode(&integers_list));

    let mut user_string = String::new();
    io::stdin()
        .read_line(&mut user_string)
        .expect("actually write something good next time");

    let mut user_string = user_string
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    let pig_latin_words: &mut Vec<String> = pig_latin_conversion(&mut user_string);
    let final_sentence = pig_latin_words.join(" ");

    println!("Your sentence converted to pig latin is {}", final_sentence);
}