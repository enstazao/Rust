// ---------------------------
//      Structs and it's types
// ---------------------------

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

fn main() {
    let mut my_car: Car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        price: 5_000,
    };

    let car_year: u32 = my_car.year;
   
    my_car.fuel_level = 30.0;    // variables are immutable by default
    let extracted_onwer: String = my_car.owner.clone();   // onwer is a heap allocated so assigning will going to change the ownership 
    println!("owner is {}", my_car.owner);   // remaning field can be accessible , this phenemenon is called  "partial move"
    // Some portion of that data has been moved out of struct instance leaving that portion invalid
    // to fix this use .clone()

    // copy most of the fields from the existance struct instance to form a new struct instance
    // if fields ..my_car will be partially moved from the given struct so remember this
    // in this case they are stack allocated fields so no issue but if heap allocated data then they will going to leave absent fields
    let another_car: Car = Car {
        owner: "new_name".to_string(),
        ..my_car
    }

    // tuple structs
    // like structs tuple can group data of different types
    let point_2D: (i32, i32) = (1, 3);
    let point_3D: (i32, i32, i32) = (4, 10, 13);

    // Issue here is that meaning of these numbers is not clear we can change the name
    // and then their meaning will be different or may in function the user it with different name
    // struct can help in these

    struct Point_2D(i32, i32);
    struct Point_3D(i32, i32, i32);

    // tuple structs are similar the difference is that their fields are not named and we use parenthsis instead of curly braces
    // tuple structs are user defined types and we can now define instances of these tuple structs

    let point1: Point_2D = Point_2D(1, 3);   // cannot use string and cannot add extra element
    let point2: Point_3D = Point_3D(4, 10, 3);

    // Unit struct
    // name with no field
    struct ABC;


}