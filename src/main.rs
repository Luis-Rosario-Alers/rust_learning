mod chapter_8;
use crate::chapter_8::{find_median, _find_mode, MedianResult};

fn main() {
    let integers_list: [i32; 6] = [1, 1, 5, 6, 6, 7];
    println!("The median for this list is {}", match find_median(&integers_list) {
        MedianResult::Float(floating_num) => floating_num,
        MedianResult::Int(int_num) => int_num as f32,
    });
    println!("The mode of this list is {}", _find_mode(&integers_list));
}