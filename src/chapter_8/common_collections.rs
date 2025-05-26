// Given a list of integers, use a vector and return the median (when
// sorted, the value in the middle position) and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.

pub enum MedianResult {
    Int(i32),
    Float(f32),
}


pub fn find_median(integers_list: &[i32; 6]) -> MedianResult {
    let integers_vector: Vec<&i32> = integers_list.iter().collect();
    if integers_vector.len() % 2 == 1 {
        let median_index = ((integers_vector.len() / 2) as f64).ceil() as usize;
        MedianResult::Int(integers_vector[median_index].clone())  
    } else {
        let median_low_index= integers_vector.len() / 2 - 1;
        let median_high_index = integers_vector.len() / 2;
        
        let median_low_value = integers_vector[median_low_index];
        let median_high_value = integers_vector[median_high_index];

        MedianResult::Float((*median_low_value as f32 + *median_high_value as f32) / 2.0)
    }
}

use std::collections::HashMap;

pub fn _find_mode(integers_list: &[i32; 6]) -> i32 {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for num in integers_list {
        let mut count = hash_map.entry(*num).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    for (key, value) in hash_map {
        if value > mode {
            mode = key;
        }
    }
    mode
}

pub fn pig_latin_conversion(strings: &mut Vec<String>) -> &mut Vec<String> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for word in strings.iter_mut() {
        if word.starts_with(vowels) {
            word.push_str("yay");
        } else if word.is_ascii() {
            let mut consonants = String::new();
            let mut punctuation = String::new();
            let mut first_vowel = false;
            word.retain(|c| {
                if !vowels.contains(&c) {
                    if c.is_ascii_punctuation() {
                        punctuation.push(c);
                        false
                    } else if !first_vowel {
                        consonants.push(c);
                        false
                    } else {
                        first_vowel
                    }
                } else {
                    first_vowel = true;
                    first_vowel
                }
            });
            word.push_str(&consonants);
            word.push_str("ay");
            word.push_str(&punctuation)
        } else {
            println!("Ngl I dont think {} is in english", &word);
        }
    }
    strings
}