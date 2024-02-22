fn main() {
    let mut vec_1: Vec<i32> = vec![4, 5, 6];

    let ref_1: &Vec<i32> = &vec_1;
    let ref_2: &Vec<i32> = &vec_1;
    // let ref_3: &mut Vec<i32> = &mut vec_1;

    println!("ref_1 is {:?} and ref_1 is {:?}", ref_1, ref_2);

}