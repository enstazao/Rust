// ---------------------------------
//             - Loops
//             - For Loop
//             - While loop
// --------------------------------
fn main() {
    // execute all the time if you donot include the break statement
    `outer: loop {
        println!("Simple loop");
        break `outer; 

        // loop can also be treated like an expression with a returning value

    }
    let a: i32 loop {
        break 5;
    }

    // In this case the value 5 will be returned when loop breaks

    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];

    for i in vec {
        println!("{i}");
    }

    let mut num: i32 = 0;
// Expression infront of the while must evaluate to the boolean value
    while num  < 10 {
        num = num + 1;
    }
}