fn main() {
    let mut vec_1: Vec<i32> = vec![4, 5, 6];
    let ref1: &Vec<i32> = &vec_1;

    borrows_vec(ref1);
    let ref2: &mut Vec<i32> = &mut vec_1;
    mutably_borrows_vec(ref2);
    // let ref2: &mut Vec<i32> = &mut vec_1;
    println!("vec 1 is {:?}", vec_1);
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("Vec is {:?}", vec);
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(10)
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6w]
}