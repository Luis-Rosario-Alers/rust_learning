
// pub fn test_double_free_error() {
//     // allocate memory on heap
//     let string1 = String::new();
//     // transfer ownership from string1 --> string2
//     // only copies stack data, not heap data.
//     let string2 = string1;
//     // string1 is now invalid.
//     
//     // this should cause an ownership error.
//     println!("{}", string1);
// }
// `drop(string2)` when we are out of scope.

pub fn test_clone() {
    // allocate memory on heap
    let string1 = String::from("hola");
    // this time we clone heap memory
    let string2 = string1.clone();
    
    // both string1 and string2 are valid.
    println!("{}", string1);
    println!("{}", string2);
}
// `drop(string1)` and `drop(string2)`

pub fn test_stack_copy() {
    // stack copy since integers are fixed scalar values.
    let num1 = 10;
    let num2 = num1;
    
    // this makes num1 and num2 still valid
    println!("num1 = {}, num2 = {}", num1, num2);
}

pub fn test_function_ownership() {
    let s: String = String::from("hola");
    dummy_function(s);
    
    // trying to do something like this 
    // println!("{s}");
    // will not work because ownership of that variable has been
    // transferred to the dummy function
    
    // this can go both ways with return values from functions.
    // transfer ownership from callee to caller
    let returned_string= dummy_return_function(); 
    println!("returned_string = {}", returned_string);
}
// `drop(returned_string)` here
fn dummy_function(string: String) -> () { 
    // transfer ownership of string to this function
   println!("{}", string);
} // string should now go out of scope here

fn dummy_return_function() -> String {
    let string: String = String::from("hola");
    string
}

fn test_references(string: &String) -> usize {
    // referencing a string instead of taking direct ownership
    string.len() // this is an example of borrowing
} // string goes out of scope here, but since it doesn't have ownership
// of the string that it's actually referencing, the referenced string isn't dropped.

fn test_immutable_reference(string: &String) {
    // this will not work because references are immutable by default
    string.push_str("nah im good.");
}

fn test_mutable_reference(string: &mut String) {
    // this will work because we are using a mutable reference.
    string.push_str("nah im good.");
}

fn test_multiple_mutable_references() {
    let mut string1 = String::from("hola");
    
    // this will cause a compilation error because
    // multiple mutable references can cause data races
    // at compile time.
    let f1 = &mut string1;
    let f2 = &mut string1;

    // meanwhile, this will not error because the mutable references
    // are in different scopes
    let f3 = &mut string1;
    {
        let f4 = &mut string1;
    }
    // essentially, they CANNOT be in the same scope
    // also, you can't have mutable and immutable references at the same time.
    // this will error in compile time.
    let f5 = &string1;
    let f6 = &string1;
    let f7 = &mut string1;
    // The reason for this is to make sure the value doesn't change
    // when an immutable reference is being used.
    // Otherwise, immutable references could point to a different value, which would render them
    // essentially pointless in terms of ensuring the immutability of the referenced value.
    
    // Furthermore, a reference's scope starts from where it was created to its last usage
    // consider the following
    let mut s = String::from("hola");
    let f8 = &s;
    let f9 = &s;
    println!("{f8} is also {f9}"); // the lifetime of reference effectively ends here.
    
    let f10 = &mut s; // you are then able to use a mutable reference.
    println!("{f10}");
}

fn test_dangling_reference() {
    // this will try to store a dangling reference from the function
    let dangling_reference = dummy_dangling_reference();
}

fn dummy_dangling_reference() -> &String {
    // allocate memory to the heap
    let a: String = String::from("hola");
    // returns a reference, but because this reference's value is going to be deallocated,
    // when it goes out of scope, it will trigger a compilation error.
    &a
    // to stop this compilation error, you have to return the string itself as it will
    // transfer ownership.
    // a
}

