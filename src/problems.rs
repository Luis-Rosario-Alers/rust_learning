


fn find_first_word(s: &String) -> usize {
    // here we make a bytes variable that
    // stores a byte array of the string
    // so that we can access the individual
    // letters of the string s.
    
    // `as_bytes()` is a method that shows the underlying representation
    // of the string in bytes.
    // This is possible because all strings in rust are UTF-8
    
    let bytes = s.as_bytes();
    
    // here we iterate through each of the bytes 
    // Ex.
    // (index, referenced value)
    
    // `iter()` iterates through collections like (arrays, vectors, etc.)
    // and gives back a reference to each element in that collection
    
    // `enumerate()` is a method that you call on an iterator (like `iter()`).
    // This will return a tuple of the index and the value, making it 
    // more convenient to keep track of the index this way.
    
    // note: we can use other methods like `chars()` instead of `as_bytes()`
    // to be able to more accurately
    // fetch non-ASCII values, but for now we will use `as_bytes()`.
    
    // With these methods in mind, we can iterate using `iter()` and then `enumerate()`
    // to wrap the result
    // of `iter()` so that each element is a tuple of the index and the value
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { 
            return i;
        }
    }
    s.len()
}

// Now looking at this function, you probably ask yourself, "is there a better way?"
// and fortunately, yes there is. 

// Instead of doing our previous method of using a byte array to find white spaces,
// potentially causing our returned index and string to be out of sync,
// we can use string splices.

// Consider the following
fn string_splices() {
    // here we define our string
    let string1: String = String::from("hola amigo");
    
    // here we create two variables that bind to 
    // different parts of string 1
    let _hola: &str = &string1[0..4];
    let _amigo: &str = &string1[5..9]; // [starting_index..ending_index]
    // these two different parts are actually just references to
    // different parts of the same string.
    
    // NOTE: string splices can be defined in multiple ways such as
    // [starting_index..ending_index]
    // [..ending_index]
    // [starting_index..]
    // [..]
}

// with string splices in mind, you can rewrite the function to the following.
fn find_first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { 
            return &s[0..i]
        }
    }
    &s[..]
}

