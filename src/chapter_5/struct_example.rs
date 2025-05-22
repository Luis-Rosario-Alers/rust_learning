use std::net::Shutdown::Read;

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// This code example calculates the area of a rectangle, but
// it has one issue. If we are calculating one rectangle's area,
// then we can make it more readable by making a more grouped
// structure.

fn main() {
    let rect = (40, 60);

    println!("The area of the rectangle is {} square pixels.", area(rect));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Again, this code is doing what it's supposed to do, but we still have to
// remember that index 0 is width and index 1 is height. We can refactor this again
// to make the meaning even clearer.

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 40,
        height: 60,
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// moving on if you want to print an instance of Rectangle
// so we can see all of its fields and debug our program, the `println!`
// macro is not going to work for this.
fn main() {
    let rect = Rectangle {
        width: 40,
        height: 60,
    };

    println!("rect is {}", rect); // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
}

// even if we try to use a debug output format
// println!("rect is {:?}", rect); error[E0277]: `Rectangle` doesn't implement `Debug`

// = help: the trait `Debug` is not implemented for `Rectangle`
// = note: add `#[derive(Debug)]` or manually implement `Debug`

// we can see that the compiler is telling us to include the
// Debug functionality manually.

// We can do this by adding the outer attribute #[derive(Debug)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Now we can print the Rectangle instance
fn main() {
    let rect = Rectangle {
        width: 40,
        height: 60,
    };

    println!("rect is {:?}", rect);
}

// rect is Rectangle { width: 40, height: 60 }

// we can make this more pretty by using {:#?} instead

// rect is Rectangle {
//    width: 40,
//    height: 60,
// }

// we can also use the !dbg macro which prints into
// the standard error console (stderr) instead of the
// standard output console stream (stdout)

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: !dbg(30 * scale), // 30 * scale = 60
        height: 50,
    };
    dbg!(&rect);
    // &rect1 = Rectangle {
    // width: 60,
    // height: 50,
}

// including the Debug trait, there are also a number
// of other traits that we can use with the derive attribute


