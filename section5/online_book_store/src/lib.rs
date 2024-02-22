// use crate::product::category; it is a private module and it is only used by parent
pub use customer::Customer;
pub use product::{Category,Product};
pub use order::Order;

// the upper level of crate can now see these modules 
mod product {

    pub use category::Category; // whenever category is used within this moduele scope then it will be replaced by this 
    pub struct Product {
        id: u64,
        name: String, 
        price: f64,
        category: Category,
    }

    mod category {
        pub enum Category {
            Electronics,
            Clothing, 
            Books,
        }
        
    }

    impl Product {
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
            Product {
                id,
                name,
                price,
                category,
            }
        }
        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }
    
        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }
    
    }
}

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Customer {
            Customer {
                id,
                name,
                email,
            }
        }
    }
}


mod order {

    use crate::product::Product;
    use crate::customer::Customer;
    pub struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }
    
    impl Order {

        pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Order {
            Order{
                id,
                product,
                customer,
                quantity,
            }
        }

        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 {
                0.1
            } else {
                0.0
            }
        }
    
        pub fn total_bill(&self) -> f64 {
            let discount: f64 = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
    
    
}


// pub(crate) mean they are visible in the current crate not visible outside the crate 
// pub(self) only visible to itself
// Two ways to map the modules into the folders 