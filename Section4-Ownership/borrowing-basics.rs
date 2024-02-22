// ------------------------------
//      Borrowing
// -----------------------------

/*
Borrowing Rules
- At any time, you can have either one mutable reference or any number of immutable references 
- references must always be valid


These rules solved two problems
    - Data races   --  
    - Dangling references
*/
fn main() {
    let mut vec_1: Vec<i32> = vec![4, 5, 6];
    // let ref1: &mut Vec<i32> = &mut vec_1;  // mutable refernce 
    // let ref2: &mut Vec<i32> = &mut vec_1;    // Cannot borrow vec_1 more than once a time

    // println!("ref1: {:?}, ref2: {:?}", ref1, ref2);     // This gives error
    // But if print line is commited then no error why ? 
    // because ref1 is before ref1 is limited to line 17 and ref2 is limited to 18 and not ref1 is their so 
    // no rules is violated
    // there is not overlap
    // with in the scope of ref1 we should not have no other mutable reference to the data 

    //    Can have many immutable references
    let ref1: &Vec<i32> = &vec_1;
    let ref2: &Vec<i32> = &vec_1;
    // let ref3: &mut Vec<i32> = &mut vec_1;  // violates the first rule as there can only be one mutable reference or any number of immutable references but not both at the same time
    //  if I move the ref3 down the print statement the code compile that is due to the scope of reference
    // during the time ref1 and ref2 are active we donot have the ref3 active so that we donot get any error 
    // By enforcing rule1 the rust remove the data races at compile time,
    // Data races occur when there are multiple references to the same data with at least one reference updating the data and there is no mechanism 
    // to synchronize access to the data

    // By enforcing these borrowing rules the rust allows to access to the data immutable many reference or mutable only one reference at at time 
    // This ensure that multiple parts of the code can have access to the data without causing the race condition 

    
    // Reference must be valid
    // let vec_2: &Vec<i32> = {
        // let vec_3: Vec<i32> = vec![4, 5, 6];
        // &vec_3
    // };  // Vec_3 does not live long enough this is due to the dangling reference




    // Both are references so they donot take the onwership only borrow the data temporarily 

}

// Borrowing is a fundamental concept in the rust ownership system
// that allows the different parts of the program to interact with data in safe and efficient manner
// Establishing a reference to some data
// Just like pointers with some rule, doesnot take the ownership 

/*
Why need borrowing
 prevent unnecessary memory usage 
 only required to read the data not require the ownership 
 it is better to provide reference rather than the clone of the data
 sometime not required the ownership so that we need borrowing borrow temporarly 


*/