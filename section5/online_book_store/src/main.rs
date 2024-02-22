
// use online_book_store::{Customer, Product, Category, Order};
// // get error that modules are private
// // 1. Fix this error by making modules public
// // create a new associated function and call that in main which we will use to give access or create struct
// // making an enum public also make it's variants public
// fn main() {
//     // let product = Product{
//     //     id: 1,
//     //     name: String::from("Laptop"),
//     //     price: 799.99,
//     //     category: Category::Electronics,
//     // };

//     let product = Product::new(1, String::from("Latpop"), 799.99, Category::Electronics);
//     let customer = Customer::new(1, String::from("Hello"), String::from("ello@gmail.com"));
//     let order = Order::new(1, product, customer, 2);
//     println!("Total cost of the order is ${}", order.total_bill());

// }


mod parent {    

    use child::Child;
    pub struct Parent {
        id: u64,
        name: String,
        child: Child,
    }

    impl Parent {
        pub fn new(id: u64, name: String, child: child::Child) -> Parent {
            Parent {
                id,
                name,
                child,
            }
        }

        pub fn print_parent_info(&self) {
            println!(
                "Parent id is {}, parent name is {}",
                self.id,
                self.name,
            )
        }
    }

    pub mod child {
        pub enum Child {
            Male,
            Female,
        }
    }
}


use parent::Parent;
use parent::child::Child;

fn main() {
    let parent_1 = Parent::new(1, String::from("ABC"), Child::Male);
    parent_1.print_parent_info();
}

// Intersection code writing is hard so what you can do is get external dependency
// crates.io 
// add the line of the dependencies or use cargo run and dependency name
// traits is a way of representing some properties for a type
// reusability advantage 
// Focus on core logic 
// Saturday avoid excessive dependency 
// use those dependencies that are easy to understand

// Publishing your crate
// cargo doc --open
// documentation comments

// add documentation comments above the item 
// /// used     private attributes not available and the variables also not added in the documentation 
