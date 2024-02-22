// ---------------------------------
//          - Variables
//              - Definition
//              - Mutability
//              - Scope
//              - Shadowing
//          - Constants
// --------------------------------

fn main() {
    // Definition
    let x: i16 = 10;
    println!("x is {x}");

    // Mutability
    let y: i16 = 10;
    // y = 10; Not assign it 


    // Scope is the code resides within the curly parenthesis
    {
        let z: i32 = 50;
    }

    // let s = z; (z is out of scope)

    // Shadowing
    let t: i32 = 10;
    let t: i32 = t + 10;
    println!("t is {t}");

    let u: i32 = 4;
    let u: f64 = 3.0;

    let v: i32 = 30;
    {
        println!("v is {v}..");
        let v: i32 = 40;
        println!("Inner v is {v}");
    }
    println!("outer v is {v}");

    // Constants
    const MAX_VALUE: u32 = 100;

}

// Notes
// Difference between the mutability and shadowing?
// Mutability allow you alter a single value but the shadowing involves the creation of two distinct variables with one of them shadowing the other.
// Shadowing is handy in situation in which you want sometimes your variables in one type and then in the other type.
// This the inner scope the variable need to used possibly with a different context.
// constants cannot be mutated
// Rust naming convention for the constants is screaming snake case. All the letters are capitalized and _ is used to separate them.
// value of the constant must be known at compile time. meaning that we cannot declare a constant without a type but no value.

