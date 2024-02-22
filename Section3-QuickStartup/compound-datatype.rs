// ------------------------------------
//              - Compound Data types
//                  - &str and String
//                  - Arrays
//                  - Vectors
//                  - Tuples
//                  - Empty Tuple
// ------------------------------------

fn main() {
    // &str: it is immutable this mean that once it is created we cannot add or remove string from it
    // String: String type allow the next to grow in size and modified
    let fied_str: &str = "Fixed length string";
    let mut flexible_str: String = String::from("This string will grow");
    flexible_str.push(ch: 's');


    // Arrays: arrays are of fixed size and once it is declared it cannnot be changed
    let mut array_1: [i32; 5] = [4, 5, 6, 7, 8];
    let num = array_1[3];

    println!("{:?}", array_1);
    let array_2: [i32; 10] = [0; 10];    // 10 elements array all intialized with the value zero
    

    // Vectors :  all elements needs to be of same type
    let vec_1: Vec<i32> = vec![4, 5, 6, 7, 8, 9];
    // specific element can be indexed same as arrays
    let num = vec_1[3];



    // Tuples: Tuples can hold values of different types
    let my_info: (&str, i32, &str, i32) = ("Salary", 40000, "Age", 40);
    let salary_value: i32 = my_info.1;
    // Destructure the tuple
    let (salary: &str, salary_value: i32, age: &str, age_value: i32) = my_info;

    let unit: () = ();
    // These unit tuples are returned implicitly when no other meaningful value can be returned
    // Functions that lack a specific value implicitly return the unit tuple
    // they are zero sized and donot consume any memory
    // Cannot multiply the u32 with i32 
    // both the variable need to be have same type to be multiplied

}