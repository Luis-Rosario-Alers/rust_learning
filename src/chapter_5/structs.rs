// to define a struct, you need to use the struct
// keyword and a name that represents the information
// the struct stores.
struct Object {
    name: String, //
    email: String,
    status: bool,
}

struct LifetimeObject {
    name: &str, //
    email: &str,
    status: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct NeverEqual;

// now let's create an instance of the struct
fn main() {
    let mut object1: Object = Object {
        name: String::from("John"),
        email: String::from("example@email.com"),
        status: true,
    };

    // to get values from a struct, you have to use dot notation.
    println!("{}", object1.name);

    // you can also change fields using this notation
    // note: this is only possible with a mutable instance
    object1.status = false;

    // we can also use functions to build structs and return them to us
    let mut object2: Object = build_object("John Doe", "example@email.com");

    // You may want to create another instance of a struct with almost the same values
    // as another but change some slightly. This can be done with Struct Update Syntax.
    let object3: Object = Object {
        name: String::from("Guy"),
        ..object2
    };
    // we use .. to say that all other values not explicitly changed should be the same as object2.
    // note: all fields from object2 that don't implement the copy trait and are not explicitly set in object2
    // are going to have ownership transferred to object3, and thus object2 will become invalid.

    // you can use tuple structs to simplify your code and give a tuple a name
    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);
    // these two tuples are different types, even though their fields include the same types

    // you can use Unit-Like structs that don't implement any fields
    let target = NeverEqual;

    // Finally, talking about the ownership of data in a struct
    // if your struct doesn't own the data (reference to data), then it will need a lifetime specifier
    // for the compiler to not error.

    // This is why the following won't work
    let lifetime = LifetimeObject {
        name: "Kevin",
        email: "example@email.com", // &str with no lifetime specifier
        status: true,
    };
}

fn build_object(name: String, email: String) -> Object {
    // if you have the same name for your parameters and fields, you can use
    // field init shorthand syntax to simplify your code.
    Object {
        name,
        email,
        status: true,
    }
}




