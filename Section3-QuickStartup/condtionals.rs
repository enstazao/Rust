// ------------------------------
//          Conditionals
//              - If else
//              - If else if ladder
//              - Match
// ------------------------------

fn main() {
    let num:  i32 = 40;
    // The conditions in if statement must evaluate to a boolean value
    // if num  // the compiler will complain in this case
    if num < 50 {
        println!("The number is less than 50");
    } else {
        println!("Th number is greater than or equal to 50");
    }

    // If else if ladder syntax
    // check condition one after the other
    let marks = '95';
    let mut grade = 'N';

    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else {
        grade = 'F';  // default case no other condition is true this one will be executed, and it is optional , it must be at the end
    }



    // It can be else if a expression 

    let grade: char = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else {
        'F'
    };

    // must need to be of same type if "A" then we will get the imcompatible type error
    // return value of same type and 
    // More than one statement in the returning then last one without a semicolon will 
    // be considered as a returing expression


    // Match
    // Match expression allow us to compare the value 
    // to series of patterns
    // and execute code based on the first matching pattern
    let marks: i32 = 95;
    // let mut grade: char = 'N';

    let grade: char = match marks {
        90..=100 =>  'A',    // First arm left side we have the pattern to match, right side we have the code block to execute in case the pattern matches
        80..=89 =>  'B',      // if multiple lines for the right side enclose them in parenthesis
        70..=79 => 'C',      // .. syntax is used to mention the range of values in rust
        _ =>  'F',             // both value are included inclusive, default arm  _ matches everything in rust, if remove this you will get the error of non-exhaustive patterns
    }



    // Each of the pattern along with it's respective code block is an arm
// Rust enforces exhaustive pattern matching which means every possible value or variant of the input type must be accounted for in the match statement
// certain common errors will be detected by the compiler immediately
// If i move the default arm to up and 70..79 down we will get the error as the one arm is not exhausting
// Unreachable pattern happens when the compiler detects that specific pattern is impossible to match because 

// Overlapping pattern 90 is included in both pattern 
// 90..=100, 80..=89 

 }