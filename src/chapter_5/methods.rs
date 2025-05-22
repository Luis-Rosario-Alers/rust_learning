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


