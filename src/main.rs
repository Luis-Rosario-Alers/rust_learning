mod chapter_8;
use crate::chapter_8::{find_median, 
    pig_latin_conversion,
    _find_mode,
    MedianResult, handle_command};
use std::{collections::HashMap, io::{self, Write}};
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

    println!("Welcome the department program...");
    let mut company_hashtable: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        print!("department (:> ");
        std::io::stdout().flush().unwrap();
        let mut user_input = String::new();

        io::stdin()
        .read_line(&mut user_input)
        .expect("Bad command.");
        
        if user_input == "quit" || user_input == "q" {
            break;
        }

        handle_command(user_input, &mut company_hashtable);
    }

}