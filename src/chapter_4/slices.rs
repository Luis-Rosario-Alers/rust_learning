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
fn find_first_word_2(s: &str) -> &str {
    // we changed the function signature to &str to be able to use
    // both &str and &String values
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

// another thing to point out before moving on is that string literals
// are string slices already because all they point to is a specific
// part of the binary.
// This allows us to pass in their references instead
// of having to write the full string slice syntax.

// Moving on, you can use the string slice syntax on other structures
// like collections.
fn test_array_slicing() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &array[1..3];

    assert_eq!(slice, &[2, 3]);
}

// this function is using the same slicing syntax to get
// the 2nd and 3rd element from this array.
// As before, the slice is just a reference to a part of an existing
// piece of data.


// So far we have learned the concepts of borrowing, ownership, and slices
// to make sure that my programs are memory safe.

