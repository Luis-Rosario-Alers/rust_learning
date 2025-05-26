// Given a list of integers, use a vector and return the median (when
// sorted, the value in the middle position) and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.

pub enum MedianResult {
    Int(i32),
    Float(f32),
}


pub fn find_median(integers_list: [i32; 6]) -> MedianResult {
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


pub fn _find_mode() {

}