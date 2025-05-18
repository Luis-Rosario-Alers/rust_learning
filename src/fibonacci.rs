

pub fn fibonacci(n: usize) -> usize {
    if n < 2 {
        return 1;
    }
    
    let mut sequence = vec![0; n];
    
    sequence[0] = 1;
    sequence[1] = 1;
    
    for i in 2..sequence.len() {
        sequence[i] = sequence[i - 1] + sequence[i - 2]; 
    }
    sequence[n-1]
}