pub fn test_loops() -> () {
    let mut count: u32 = 0;
    
    // testing loop labels. 
    // note: coming from python, this is actually amazing.
    // no more flags😭
    'first_level: loop {
        println!("first_level iteration");
        loop {
            println!("second_level iteration");
            if count == 2 {
                break 'first_level;
            } else {
                break;
            }
        }
        count += 1;
        println!("current count = {}", count);
    }
    
    println!("final count = {}", count);
    
    // testing while loops
    while count != 5 {
        count += 1;
        println!("count is {}", count);
    }
    
    // testing array access with loops
    
    // for arrays, you can either declare [type: length]
    let _annotated_array: [i32; 5] = [1, 2, 3, 4, 5];
    // or [element: length]
    let array = [5; 5];
    
    for element in array {
        println!("element is {}", element);
    }
    
    // testing for loops with ranges
    
    // note: ranges have differences in exclusivity
    // doing a..b is inclusive to exclusive
    // but doing a..=b is inclusive to inclusive
    
    // other things to note are that any ranges that start
    // from greater than to less than DO NOT include anything in their
    // range because ranges go from start to finish
    
    // Ex. 4..1 --> () no range.
    // Ex. 4..=1 --> () no range.
    // Ex. 1..4 --> 1, 2, 3.
    // Ex. 1..=4 --> 1, 2, 3, 4.
    
    // To iterate backwards, you should use `.rev()` to start higher and end lower.
    // All rules still apply while using `.rev()`.
    
    // Ex. `(1..4).rev()` --> 3, 2, 1.
    // Ex. `(1..=4).rev()` --> 4, 3, 2, 1.
    
    for number in (1..4).rev() {
        println!("{}", number);
    }
    
    
    println!("Yay! we're done testing loops.");
}