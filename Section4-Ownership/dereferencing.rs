// ------------------------------
//          Dereferencing
// ------------------------------
fn main() {
    let mut some_data: i32 = 42;
    let ref_1: &mut i32 = &mut some_data;

    let dref_copy: i32 = *ref_1;

    *ref_1 = 43;  // some_data will be updated to 42
    println!("some_data is : {some_data}, and drefCopy is : {dref_copy}");

    // Heap allocated data behaves differently 
    let mut heap_data: Vec<i32> = vec![5, 6, 7];
    let ref_1: &mut Vec<i32> = &mut heap_data;

    let dref_copy: Vec<i32> = ref_1.clone;   // this creates a copy not see it is a reference or anything but creates a copy





    // Two issues
    // 1. we are assigning using ref_1 which is not a owner it is a mutable reference only 
    // 2. Moving a value out of the mutable reference could potentialy leave the reference invalid 


    // Mutable reference to some data is copied only once 
    let move_out: &mut Vec<i32> = ref_1;
    // let move_out_again: &mut Vec<i32> = ref_1;

    // mutable reference are moved not copied 
    // cannot create copied of the mutable reference 
    // but for the immutable reference I can create multiple copies of the immutable reference 
    // Remember the borrowing rules we can have mutiple immutable references but only one mutable reference 

}

/*
You need to dereference to referenced value to access the underlying data

 */