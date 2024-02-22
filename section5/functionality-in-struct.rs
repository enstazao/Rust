struct Car {
    owner: String, 
    year: u32, 
    fuel_level: f32,
    price: u32,
}

impl Car {

    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
        // colon colon syntax is used to call the associated functions
    }

    // A frequently observed pattern is rust is for types to include an associted function 
    // named as new that served as a constructor function
    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year: year,
            fuel_level: 0.0,
            price: 0,
        }
    } 
     
    fn display_car_info(&self) {
        println!(
            "Onwer: {}, Year: {}, Price: {}",
            self.owner, self.year, self.price
        );
    }

    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    fn sell(self) -> Self {
        self
    }
}

fn main() {
    let mut my_car: Car = Car {
        owner: String::from("ABC"),
        year: 2019,
        fuel_level: 0.0,
        price: 30
    };

    my_car.display_car_info();

    my_car.refuel(10.5);

    // my_car is no longer accessible and the ownership is transferred to the new_owner
    let new_owner: Car = my_car.sell();
    // my_car.refuel(10.0);  borrow of moved value
    new_owner.display_car_info();

    let new_car: Car = Car::new(String::from("XYZ"), 2020);
    new_car.display_car_info();

    // dipslay_car_info(&car_info) not found in scope
    // 2 conditions to be consider function as a method
    // 1. It has to be inside implementation block of the type on which it is defined
    // 2. first parameter must be self
    // 3. Three types of self
    // 1. immutable form which we have in the above display_car_info function (&self) useful when we are only reading the data but doesnot modify it
    // 2. mutable reference to self for instance which will refuel the car
    // 3. last form of self is the method can take is the old form of self this will take the ownership from the caller of the instance and return it
    // to the new instance of the car, since the method need ownership so the input parameter is self
    // In the implementation block the self with the capital S refers to the type for which the method is defined in this case the type car
    // we can use the Car but using Self is more preffered
    // this is used when you convert one type to another type while ensuring that the orignal instance remain inaccessible to the caller  


    // Now talk about the associated functions or static function in other programming languages these functions are connected to the type 
    // itself but donot operate on the instances of that type 
    // associated functions donot take self as an argument 
    // associated functions are not called using the dot syntax

}