
pub fn convert_temperature(temp: &mut f64, temp_type: bool) -> () {
    // F to C
    if temp_type {
        *temp = (*temp - 32.0) / 1.8
    } else { // C to F
        *temp = (*temp * 1.8) + 32.0
    }
}