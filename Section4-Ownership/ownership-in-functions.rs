 // -----------------------------------
//      Ownership in Functions
// -----------------------------------

fn main() {
    // let vec_1: Vec<i32> = vec![1, 2, 3];
    // takes_ownership(vec_1); use .clone so that it can easily pass 
    // println!("vec 1 is {:?}", vec_1); // this will give error same as assigning the vector to a variable
    // ownership is transferred from vec_1 to vec when we passed that to a function

    let vec_2: Vec<i32> = gives_ownership();
    println!("Vec 2 is : {:?}", vec_2);

    let vec3: Vec<i32> = gives_and_take_ownership(vec_2);

    // println!("Vec2 is {:?}", vec_2); // tell what's the error here
    println!("vec3 is {:?}", vec3);

    let var: i32 = 10;
    stack_function(var);
    println!("var is main is {var}");

}

fn stack_function(mut var: i32) {
    var = 56;
    println!("fun var is {var}");
}
// fn takes_ownership(vec: Vec<i32>) {
//     println!("vec is {:?}", vec);
//     // at the end of function it will be cleaned up from heap
// }

fn gives_ownership()  -> Vec<i32> {
    vec![4, 5, 6]
}

fn gives_and_take_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}