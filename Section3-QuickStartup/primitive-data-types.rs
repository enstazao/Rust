// ---------------------------------------
//          - Scalar Data Types
//              - Integers
//              - Floats
//              - Chars
//              - Boolean
// ---------------------------------------
fn main() {
    // Unsigned Integers (positive integers start with letter u followed by number of bits it can store)
    let unsigned_num: u8 = 5;   // u16, u32, u128

    // Signed Integers (start with the letter i i32 is the default type for integers in rust)
    let signed_num: i8 = 5;

    // Floating Point Numbers (number with decimal places)
    let float_num: f32 = 5.0; // f32, f64

    // Platform specific integers
    let arch_1: usize = 5;     // Pointer sized unsigned integer
    let arch_2: isize = 5;     // Pointer sized signed integer

    // Characters   'character value must be enclosed in single quotes
    let char: char = 'a';

    // Boolean
    let b: bool = true;
    

    // Type aliasing
    // Type aliasing is a new name for the existing type
    // It is created with the type keyword followed by the name for the type
    // We use type aliasing to enhance readability
    type Age = u8;

    let peter_age: Age = 42;


    // Type Conversion 
    // Type conversion is required in precision in computation or to overcome the compatibility 
    // issue in the different part of the code
    let a: i32 = 10;
    let b: f64 = a as f64;
    

}