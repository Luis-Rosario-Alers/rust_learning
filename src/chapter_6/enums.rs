// Chapter 6: Enums and Pattern Matching

// An enum is a sum type that can be one of several possible variants, each of which 
// can hold different types of data.

// =============================================================================
// BASIC ENUM DEFINITION
// =============================================================================

// We want to make a data structure that represents either an IPv4 or IPv6 address
enum IpAddr {
    V4,
    V6,
}
// V4 and V6 are the variants of the enum IpAddr

// If you notice we are using the double colon (::) to access the variants of the enum
// this is similar to how we access associated functions in structs
// Also, since enums are types, we can make functions that take them as parameters
// this makes enums pretty useful for pattern matching as seen here.
fn print_ip(ip: IpAddr) {
    match ip {
        IpAddr::V4 => println!("This is an IPv4 address"),
        IpAddr::V6 => println!("This is an IPv6 address"),
    }
}

// This function can be called either with an IPv4 or IPv6 address
fn basic_enum_example() {
    let ip_v4 = IpAddr::V4;
    let ip_v6 = IpAddr::V6;

    print_ip(ip_v4);
    print_ip(ip_v6);
}

// =============================================================================
// ENUMS WITH DATA
// =============================================================================

// Now if you notice, V4 and V6 are empty, they hold no data.
// But what if we want to store some data in the enum variants?

// We can just define our enum like this instead
enum IpAddrWithData {
    V4(String), 
    V6(String),
}
// The variants V4 and V6 now have associated String values

// Also this makes it easier to understand that 
// the variants in the enum also construct an instance of the enum
// IpAddrWithData::V4(String) <-- makes an instance of the enum and takes String as an argument
// We automatically get constructors for each variant of the enum

fn enum_with_data_example() {
    // Now we can store a string in each variant of the enum
    let ip_v4 = IpAddrWithData::V4(String::from("127.0.0.1"));
    let ip_v6 = IpAddrWithData::V6(String::from("::1"));
}

// Another advantage of using enums is that each variant can have different types of data
// so if I want to store V4 as 4 u8 values and still have V6 as a String, I can do that too
// This is something that would not be possible with structs
enum IpAddrWithMixedData {
    V4(u8, u8, u8, u8), // 4 u8 values for IPv4
    V6(String),          // String for IPv6
}

// Not surprisingly, the demand for encoding and storing IPv4 and IPv6 addresses
// is so common that Rust already has a standard library type for this purpose
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddrStdLib {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
// You can see that the IpAddr enum now has two variants, V4 and V6,
// each of which contains a struct that represents the respective IP address type.

// This is to show that enums can store just about anything.
// From numeric types, string, structs, or even other enums.

// Note that we can still create our own definition without conflict because
// we haven't brought the standard library types into scope.

// Let's look at another example of an enum that can hold data
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Struct-like variant with named fields
    Write(String),             // Tuple-like variant with a single String
    ChangeColor(i32, i32, i32),// Tuple-like variant with three i32 values
}

// The variants in this enum are like defining different structs without using the struct keyword.
// Also, all the variants are grouped together under the Message type.
// This allows us to create a single type that can represent different kinds of messages,
// each with its own associated data.

// =============================================================================
// IMPLEMENTING METHODS ON ENUMS
// =============================================================================

// Also, similar to struct, we can implement methods for enums as well
impl Message {
    fn call(&self) {
        // Here we can define what happens when we call this method on a Message instance
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn message_enum_example() {
    let m = Message::Write(String::from("Hola"));
    m.call(); // This will call the method defined for the Message enum
}

// =============================================================================
// THE OPTION<T> ENUM
// =============================================================================

// Now let's look at another enum that is useful in Rust: Option.
// The Option enum is simply an enum that can either represent a value of some type or no value at all.
// It is defined in the standard library as follows:
/*
enum Option<T> {
    Some(T), // Contains a value of type T
    None,    // Represents no value
}
*/

// The Option enum and its variants are included in the prelude
// so you can use them without needing to import anything.
// The <T> syntax is a generic type parameter,
// which means that Option can hold a value of any type.

fn option_examples() {
    let some_number = Some(5);        // Option<i32> with a value of 5
    let some_char = Some('e');        // Option<char> with a value of 'e'
    let absent_number: Option<i32> = None; // Type annotated to Option<i32> with no value

    // Now you may ask yourself, why is Option<T> better than having null?
    // The main reason is that Option<T> forces you to handle the case where there is no value.

    // So for example, you cannot use an Option<T> without checking if it is Some or None first.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // This will not compile - rust doesn't understand how to add an i8 and an Option<i8>.
    // let sum = x + y;

    // Instead, you have to explicitly handle the Option type:
    let sum = match y {
        Some(value) => x + value,
        None => x, // or handle the case where y is None
    };
    
    println!("Sum: {}", sum);
}

// =============================================================================
// PATTERN MATCHING WITH MATCH
// =============================================================================

// If you noticed, we used a control flow construct called match.
// Match is a control flow that allows you to compare a value against a series of patterns
// and execute code based on which pattern matches.
// It is similar to a switch statement in other languages but more powerful.

// Consider the following example:
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // This function takes a Coin enum as a parameter
    // and returns its value in cents.
    match coin {
        Coin::Penny => 1,    // Arm 1
        Coin::Nickel => 5,   // Arm 2
        Coin::Dime => 10,    // Arm 3
        Coin::Quarter => 25, // Arm 4
    }
}
// The match statement compares the value of coin against each variant of the Coin enum
// and returns the corresponding value in cents.

// You can also expand to multiple lines by using {} in the match arms
fn value_in_cents_verbose(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is a penny");
            1
        }
        Coin::Nickel => {
            println!("This is a nickel");
            5
        }
        Coin::Dime => {
            println!("This is a dime");
            10
        }
        Coin::Quarter => {
            println!("This is a quarter");
            25
        }
    }
}

// =============================================================================
// PATTERNS THAT BIND VALUES
// =============================================================================

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(State), // Quarter variant now holds a State enum
}

// We can also use patterns that bind values to variables in the match arms.
// This allows us to extract data from the enum variants
fn value_in_cents_with_state(coin: CoinWithState) -> u8 {
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => { 
            // This arm matches the Quarter variant and binds the state value
            println!("This is a quarter from {:?}", state); // We then print the state
            25
        }
    }
}

// Now going back to the Option<T> enum,
// we can use it to handle cases where a value might be absent.
fn option_matching_example() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    // Using match to handle Option<T>
    match some_number {
        Some(value) => println!("We have a number: {}", value), // matches Some variant
        None => println!("No number found"),
    }

    match absent_number {
        Some(value) => println!("We have a number: {}", value),
        None => println!("No number found"), // matches None variant
    }
}

// =============================================================================
// EXHAUSTIVE MATCHING AND CATCH-ALL PATTERNS
// =============================================================================

// Moving on, there is another aspect of match that is worth noting.
// The arms' patterns must cover every possible value of the enum.
// If you don't cover all possible values, the compiler will throw an error.

// Consider the following example:
enum Color {
    Red,
    Green,
    Blue,
}

// This would not compile because we didn't handle the Color::Blue variant
/*
fn print_color_incomplete(color: Color) {
    match color {
        Color::Red => println!("This is red"),
        Color::Green => println!("This is green"),
        // Color::Blue => println!("This is blue"), // Uncommenting this would make the code compile
    }
}
*/

// This is true for all enums, and it ensures that you handle all possible cases,
// which is a good thing because it prevents runtime errors due to unhandled cases.

// You can use a catch-all pattern to handle any remaining cases
fn print_color_with_catchall(color: Color) {
    match color {
        Color::Red => println!("This is red"),
        Color::Green => println!("This is green"),
        _ => println!("This is some other color"), // Catch-all pattern
    }
}

// Note that using _ as a catch-all pattern will mean that you can't access the value of the matched variant.
// If you want to access the value, you can use a variable pattern
fn print_color_with_variable(color: Color) {
    match color {
        Color::Red => println!("This is red"),
        Color::Green => println!("This is green"),
        other_color => println!("This is some other color: {:?}", other_color), // Accessing the value
    }
}

// Finally, you can use the () unit type to have no code run in a match arm.
fn print_color_with_unit(color: Color) {
    match color {
        Color::Red => println!("This is red"),
        Color::Green => (), // No code runs for this arm
        Color::Blue => println!("This is blue"),
    }
}

// =============================================================================
// IF LET SYNTAX
// =============================================================================

// Moving on to if let syntax. 
// if let syntax allows you to look for one pattern and execute code if it matches.
// This is useful when you only care about one specific case of an enum,
// and you don't want to write a full match statement for it.

// Consider the following example:
fn if_let_example() {
    let some_number = Some(5);

    if let Some(value) = some_number {
        println!("We have a number: {}", value); // This will print the value if it is Some
    } else {
        println!("No number found"); // This will not run because some_number is Some(5)
    }
}
// The if let syntax is a shorthand for matching a single pattern.

// =============================================================================
// MAIN FUNCTION - DEMONSTRATING ALL CONCEPTS
// =============================================================================

fn main() {
    println!("=== Basic Enum Example ===");
    basic_enum_example();
    
    println!("\n=== Enum with Data Example ===");
    enum_with_data_example();
    
    println!("\n=== Message Enum Example ===");
    message_enum_example();
    
    println!("\n=== Option<T> Examples ===");
    option_examples();
    
    println!("\n=== Pattern Matching Example ===");
    let coin = CoinWithState::Quarter(State::Alaska);
    println!("Value: {} cents", value_in_cents_with_state(coin));
    
    println!("\n=== Option Matching Example ===");
    option_matching_example();
    
    println!("\n=== if let Example ===");
    if_let_example();
}

// =============================================================================
// SUMMARY
// =============================================================================

// To summarize, we know how to define enums, use them, and implement methods for them.
// We know how to use enums with data, and we know how to use the match control flow construct to handle different cases of an enum.
// We also know how to use the Option<T> enum to handle cases where a value might be absent,
// and we know how to use if let syntax to handle a single case of an enum.

// Your rust programs will now be more robust and type-safe with the use of enums and structs.

// Now to give your users an organized and straightforward API that exposes exactly what they will need, we are going to use modules.