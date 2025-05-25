// Chapter 7: Managing Modules, Packages, and Crates.

// As the programs you write get bigger, you will need to worry about grouping code
// by functionality and features. This will mean that you will have to clarify where
// a particular feature is located and where to go to change how that feature works

// This means that you will have to organize your code into multiple modules,
// then into multiple files. Finally, you can extract parts into separate crates
// that become external dependencies.

// So when you have a project that has multiple interrelated dependencies, Cargo
// provides something called workspaces.

// Rust has a number of features that allow you to chose which parts of your code
// is exposed, the organization of that code, and what are the names of each scope in
// your program. These features collectively make the module system in Rust.

// Below is list of feature definitions

// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

// We will explain further how to use all of these features

// =======================
// Packages and Crates
// =======================

// First, lets talk about crates

// A crate in its simplest form is just the smallest amount of code that the
// Rust compiler processes at a time. So even if you run `rustc` instead of `cargo`
// and pass just one file, the compiler will still treat it as a crate.

// A crate can contain other modules within it that could be defined in other files,
// which will be compiled with the crate in the final binary.

// A crate can come in two forms

// A binary crate which compiles into a binary, like a CLI program.

// A library crate which is not meant to be an executable but rather
// give functionality to other programs, an example of this would be the
// `rand` crate that was used very early into this book.

// Rustaceans use crate interchangeably with the concept of a library

// the crate root is a source file that the Rust compiler starts from to make
// up the root module of your crate

// Now to talk about packages

// a package is a bundle of one or more crates that provides a set of functionality.
// a package contains a `Cargo.toml` file that defines how to build those crates.

// a package can contains as many binary crates as it wants, but only one library crate.
// Also a package must contain at least one crate, whether it is a binary or library crate.

// Now to give further perspective to how this all plays into a project,
// we can look at the command `cargo new fake-project`

// Cargo new fake-project
//  Created binary (application) `fake-project` package
// $ ls fake-project
// Cargo.toml
// src
// $ ls fake-project/src
// main.rs

// after we make the new project, we use ls to see what is inside of the project
// and what we see is just a `Cargo.toml` file and src folder that contains `main.rs`.

// Cargo follows the convention that src/main.rs is the crate root of a binary crate.
// This is the same for library crates when Cargo sees src/lib.rs, it knows that the 
// crate root of this library crate is there. 

// Cargo will then pass the crate root to rustc to build the binary or library.

// Anyway, since we see that our project currently has src/main.rs, Cargo knows
// that this package currently contains one binary crate. Now if we decided to add
// something like src/lib.rs then Cargo would understand that we have a binary crate
// and library crate in our package.

// If we wanted to add more binary crates, 
// we could place files in the src/bin directory
// each file in that directory would be a separate binary crate. 


                                  /* =======================
                                    Module System Cheat Sheet
                                     ======================= */

// FILE STRUCTURE & CRATE ROOT
// Binary crate: Compiler starts at src/main.rs
// Library crate: Compiler starts at src/lib.rs

// DECLARING MODULES
// In crate root file, declare with: mod module_name;
// Compiler looks for code in this order:
// 1. mod module_name { /* inline code */ }
// 2. src/module_name.rs
// 3. src/module_name/mod.rs

// DECLARING SUBMODULES
// In any other file, declare with: mod submodule;
// Compiler looks in parent module's directory:
// 1. mod submodule { /* inline code */ }
// 2. src/parent/submodule.rs
// 3. src/parent/submodule/mod.rs

// PATHS TO CODE
// Access items using full path: crate::module::submodule::Item

// PRIVACY RULES
// Private by default: Code in modules is private from parent modules
// Make public: Use pub mod instead of mod
// Public items: Use pub before declarations within public modules

// THE use KEYWORD
// Create shortcuts to avoid long paths:
// use crate::garden::vegetables::Asparagus;
// Now just use: Asparagus instead of full path

// EXAMPLE STRUCTURE
// backyard/
// ├── Cargo.toml
// └── src/
//     ├── main.rs           // crate root
//     ├── garden.rs         // module
//     └── garden/
//         └── vegetables.rs // submodule

// KEY FILES CONTENT
// main.rs: pub mod garden; + use statements
// garden.rs: pub mod vegetables;
// vegetables.rs: pub struct Item {}

// This creates a hierarchy: crate::garden::vegetables::Item

/* =======================
    Defining Modules to Control Scope and Privacy
    ======================= */

// So essentially when you define modules, the code in that module is ALWAYS private.
// This means that you control what details get exposed.
// Now you may ask, why is this even useful?

// The reason this is useful is because it allows you hide implementation details
// from users to only expose the interface that they need.

// This allows for clean interfaces that are future proof and prevent misuse of code like
// accidentally calling an internal function.

// To fully realize this concept, we can think of a banking system.
// Now you have certain things that you would like your users to be able to do, for example,
// withdrawing, depositing, opening and closing account, etc..

// These functions (or more likely endpoints if on a server) are going to be public for the user to
// interface with. Now think about the operations that would need to occur in the background
// for these actions to be realized, maybe you have some internal function that handles
// database connections. This function is something that you would NOT want your users
// to accidentally call because it could cause catastrophic errors for your users and for your
// business. This is why having the ability to make specific functionality public or private
// is so important. 

// For example:
// pub fn deposit(amount: f64) { ... }           // ✅ Safe for users
// fn connect_to_database() { ... }              // ❌ Internal only
// fn bypass_fraud_detection() { ... }           // ❌ Would be catastrophic!

// While we're focusing on Rust library modules here, these same privacy 
// principles are critical everywhere - especially in servers where 
// accidentally exposing internal functions can be catastrophic.

// Now to continue, how can we scope out certain functions so that this DOESN'T occur?

// Consider the following

mod BackEnd {
    mod Payments {
        fn deposit() {}
        fn withdraw() {}
    }
    mod Accounts {
        fn open_account() {}
        fn close_account() {}
        fn freeze_account() {}
    }
}

// Note: modules can hold a multitude of things inside of them, including other modules, structs
// enums, constants, traits, and functions.

// So essentially what we did here is that we make the `BackEnd` module,
// which contains two other modules that are called `Payments` and `Accounts`.
// This structure of scoping out functions into their specific modules allows us
// to more carefully grant public access of functions that we want our users to be able
// to interact with.

// In addition, this scoping out of code also gives developers a easier time finding the associated code
// for a certain piece of functionality. It helps keep the project organized as it grows.

// To continue, remember how we talked about crate roots? specifically
// src/main.rs for binary crates and src/lib.rs for library crates

// Well the reason for their name is that the contents of these two files form a module
// at the top of the module tree called crate

// here is an example of what I mean


// Example module tree:
// crate
//  └──BackEnd
//     ├── Payments
//     │    ├── deposit
//     │    └── withdraw
//     └── Accounts
//         ├── open_account
//         ├── close_account
//         └── take_payment

// As you can see, there is an implicit crate module at the top of the tree
// and there is the `BackEnd` module that has two children `Payments` and `Accounts`.
// These two modules are called siblings since they are both under the same parent module.

// If you think about it, the module tree basically like file explorer on your computer.

/* =======================
    Paths for Referring to an Item in the Module Tree
   ======================= */

// To access items (functions, structs, etc.) inside modules, you use paths.
// Paths show the route through the module tree to the item you want.

// There are two main types of paths:
// 1. Absolute paths: Start from the crate root using the `crate` keyword.
// 2. Relative paths: Start from the current module and use `self`, `super`, or just the item/module name.

// Both are followed by one or more of identifiers and separated by double colons (::).

// Example module tree:
// crate
//  └──BackEnd
//     ├── Payments
//     │    ├── deposit
//     │    └── withdraw
//     └── Accounts
//         ├── open_account
//         ├── close_account
//         └── take_payment

// Absolute path example:
// crate::BackEnd::Payments::deposit()

// Relative path example (from inside BackEnd):
// Payments::deposit()

// You can use `super` to refer to the parent module, and `self` to refer to the current module.

// Example using `super`:
// If you're inside BackEnd::Payments and want to call a function in BackEnd::Accounts:
// super::Accounts::open_account()

// Paths are how you navigate and access code in Rust's module system.

// Now to clarify situations like accessing functions that are inside of modules inside modules
// from another module

mod backend {
    mod payments {
        fn deposit() {
            println!("Depositing!");
        }
    }
    pub mod accounts {
        pub fn open() {
            println!("Opening account!");
        }
        pub fn deposit_via_payments() {
            // Absolute path
            crate::backend::payments::deposit();
            // Relative path
            super::payments::deposit();
        }
    }
}

fn main() {
    backend::accounts::open();
    backend::accounts::deposit_via_payments();
}

// this code will NOT compile because there is one simple error that we are making.
// We cannot use the functions inside of payments because the module is not specified as public.

// To understand when we can and cannot use functions, understand this principle.

// When we are inside the scope of a module, we can define other modules, and those other modules can define functions.
// Now the thing is that when you are in the scope of the parent module in that situation, you cannot use the functions
// defined in the child module because they are hidden within the scope of the child module. Now if you are operating in the
// scope of the child module, the implementation details of your parents are completely visible to you because you see the context
// in which you are defined in.

// parents -/-> child
// parents <--- child

// Now as we learned before, you can use the `pub` keyword on the items inside
// child module to expose implementation details to the parent module

/* =======================
    Exposing Paths with the pub Keyword
   ======================= */

// Okay so you may think that since we can use the keyword `pub` to make items public
// we can just do something like this

mod BackEnd {
    pub mod Payments {
        fn deposit() {}
        fn withdraw() {}
    }
    mod Accounts {
        fn open_account() {}
        fn close_account() {}
        fn freeze_account() {}
    }
}

fn main() {
    crate::BackEnd::Payments::deposit()
}

// This will still not work even if we make the module public.
// The reason why this didn't work is because when you make a module public,
// you are essentially just allowing code to reference it, not its inner
// implementation.

// Since modules are containers for code, we cant do much other than making individual
// items in the module public

mod BackEnd {
    pub mod Payments {
        pub fn deposit() {}
        fn withdraw() {}
    }
    mod Accounts {
        fn open_account() {}
        fn close_account() {}
        fn freeze_account() {}
    }
}

fn main() {
    crate::BackEnd::Payments::deposit()
}

// Now the code compiles

// Let's explain why
// So first we start of with crate and then refer to the `BackEnd` module, now even though the `BackEnd` module isnt
// public, because main is also defined within the same module as `BackEnd` (meaning that they are sibilings), we are allowed to
// refer to `BackEnd` from the main function. Next, we see that `Payments` is marked with `pub` so we can access `Payments`.
// After accessing `Payments`, we get to the `deposit` function that is marked with `pub`, allowing us to call it.

// All of this is very important if you plan to release crates to the public because the way your API is designed determines
// how dependable your project is for other to use.

// Note: There is the popular strategy of having a binary crate and library crate within the same package to be able to test
// library implementations as if you were the client. This helps with library development tremendously as it allows you to iron
// out potential inner implementation leaks that the user shouldn't have access to.

// Finally for some minor concepts to cover.

// Example usage of super
mod outer {
    pub fn hello() {
        println!("hola outer");
    }

    pub mod inner {
        pub fn call_outer() {
            super::hello();
        }
    }
}

fn main() {
    outer::inner::call_outer();
}

// All super does is reference the parent module, giving you access to the parent module's inner implementation functions.

// Example of making structs public
pub struct Object {
    name: String, // Field still private
    desc: String,
    id: u32,
}

impl Object {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            id: 5,
        }
    }
}

fn main() {
    let obj = Object::new();
    println!("name of Object is {}", obj.name);
}

// This will not work because when you make structs public,
// you are making the structure itself public, not its fields.

// This is the opposite for enums, which when made public immediately exposes all of
// its variants as well.

// This is already the default in Rust anyway but it has to be said.

// Now lets talk about the `use` keyword, really all it does it just give you a shortcut to the
// path you want to reference

// Example usage of `use`
mod BackEnd {
    pub mod Payments {
        pub fn deposit() {}
        fn withdraw() {}
    }
    mod Accounts {
        fn open_account() {}
        fn close_account() {}
        fn freeze_account() {}
    }
}

use crate::BackEnd::Payments;

fn main() {
    Payments::deposit()
}

// we just made the shortcut to the payments path making it easier to access those functions.
// use will only be valid in the scope that it is in

// so something like this will not work
mod BackEnd {
    pub mod Payments {
        pub fn deposit() {}
        fn withdraw() {}
    }
    mod Accounts {
        fn open_account() {}
        fn close_account() {}
        fn freeze_account() {}
    }
}

use crate::BackEnd::Payments; 

mod fake_mod {
    pub fn fake_main() {    
    Payments::deposit() // Not in the same scope as the use statement
    }
}

fn main() {
    crate::fake_mod::fake_main()
}

// Lastly, to make your usage of `use` idiomatic
// you want to bring in the parent module into scope instead of the function directly
// to signify that the function you are calling is not defined in the scope you are currently in.

// This does not apply to things like structs, enums, and other items with `use`, as its idiomatic to specify the
// full path.

// Example of idiomatic usage of `use`
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// The only exception to this convention is when we are bringing two items into scope with the same
// name using `use` statements, because Rust doesn't allow this. 

// Example of exception
use std::fmt;
use std::io;

// We bring in two Result types from two different parents
fn function1() -> fmt::Result {
//    --snip--
}
fn function2() -> io::Result<()> {
//    --snip--
}

// We could also just use as to give a type a local name

// Example of alias
use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result {
//   --snip--
}
fn function2() -> IoResult<()> {
//   --snip--
}

