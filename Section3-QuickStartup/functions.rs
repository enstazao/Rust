// ---------------------------------
//              - Functions
//              - Code Blocks
// ---------------------------------

fn main() {
    // Functions in rust are declared using the fn keyword followed by the function name
    my_fn("Hello World!");
    let str: &str = "Function call with a variable";
    my_fn(str);

    let answer: i32 = multiplication(num1: 10, num2: 16);

    let result: (i32, i32, i32) =  basic_math(num1: 10, num2: 17);

    let (multiplication: i32, addition: i32, subtraction: i32) = basic_math(num1: 10, num2: 17);

    // Code blocks
    // Code blocks are the statements enclosed in the curly parenthesis

    let full_name: String = {
        let first_name: &str = "Nouman";
        let last_name: &str = "Azam";

        // Like functions code blocks also have a return value

        // format is used for string formatting in rust
        // like functions in the code blocks the last expression in the code block without the semi colon will 
        // be the returning value
        format!("{first_name} {last_name}")
    };

    // While code block is an assignment statement so we will add the semicolon at the end
    // Code blocks share similarties just like the functions 
    // They have their own separate body and they also return value
    // They have variables which are limited to the scope in their body
    // Key difference: 1. code blocks are not designed for reused 2. Code blocks donot have any explicit parameters
    // All the variables in which code block lies that are only visible to it

}

// naming convention for the functions is snake case

fn my_fn(s: &str) {
    println!("{s}");
} 


fn multiplication(num1: i32, num2: i32) -> i32 {
    // return 45;
    println!("Computing multiplication");
    num1 * num2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}

// Experssions are the code lines that evaluate to a returning value last code line is the expression since it return the value
// Statements are the instruction that donot return any value println is a statement it does not return any value

// If we add the ; at the end we get the mismatch error what we are returning the unit tuple but it is expecting an i32 integer in return
// There has to be one and only one returning expression cannot be multiple and that needs to be the last expression in the function
// If we want to return early we will have to add the return keyword with the proper value
// matching the return type