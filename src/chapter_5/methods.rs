// Methods are essentially functions that
// are defined with a structure in mind

// Their first parameter will always be self, which
// represents the instance of the struct that the method is
// called in

// to make a method for the previous Rectangle struct,
// we can define the area method on the Rectangle struct.

#[derive(Debug)] // Implements Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// if we want to implement methods that change the instance,
// we can do &mut self, and if we want to transfer ownership of the instance, we can do self

fn main() {
    let rect = Rectangle {
        width: 40,
        height: 60,
    };

    println!("The area of the rectangle is {}", rect.area());
}

// we can also make methods with the same names as fields

impl Rectangle {
    // methods that retrieve some type of field from the struct
    // are called getters, they are useful for implementing read-only access
    // to a field by making the field private and the method public.
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 40,
        height: 60,
    };
    // depending on if we use parentheses or not, it changes what we are referring to.
    // parentheses mean method and no parentheses mean field
    println!("The height of the rectangle is {}", rect.height); // referring to field
}

// now let's try adding methods with more parameters
impl Rectangle {
    // we have &self and other as parameters
    // &self is the instance of the struct that the method is called on
    // other is another instance of the struct that we want to compare with
    // this method will check if the current rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// Now let't talk about associated functions
// Associated functions are functions that are associated with a struct
// but do not take self as a parameter

// consider the following

impl Rectangle {
    // this function will create a new instance of the Rectangle struct
    // it is called an associated function because it does not take self as a parameter
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    // we would access this function using the struct name
    // followed by two colons (::)
    let rect = Rectangle::new(30, 50);
    println!("The area of the rectangle is {}", rect.area());
}

// the reason you use two colons is because the associated function
// is namespaced by the struct it is in. This two colon (::) syntax is
// used for associated functions and namespaces created by modules.

// Moving on, we can use multiple impl blocks to implement methods
impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// This is syntactically valid and there are use cases for this
// which will be explored in the later chapters of the book. 