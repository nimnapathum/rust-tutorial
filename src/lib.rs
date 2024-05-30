pub use crate::product::Product;
pub use crate::product::Category;
pub use crate::customer::Customer;
pub use crate::order::Order;

mod product{
    pub use category::Category;
    pub struct Product {
        id: u64,
        name: String,
        price: f64,
        category: Category,
    }
    
    mod category{
        pub enum Category{
            Electronics,
            Clothing,
            Books,
        }
    }
      
    impl Product{
        fn calculate_tax(&self) -> f64{
            self.price * 0.1
        }
    
        pub fn product_price(&self) -> f64{
            self.price + self.calculate_tax()
        }

        pub fn new(id: u64 , name: String , price: f64 , category: Category) -> Product{
            Product { 
                id, 
                name,
                price, 
                category
            }
        }
    }
}

mod customer{
    pub struct Customer{
        id: u64,
        name: String,
        email: String,
    }
    
    impl Customer{
        pub fn new(id: u64 , name: String , email: String) -> Customer{
            Customer{
                id,
                name,
                email
            }
        }
    }
}

mod order{
    use crate::product::Product;
    use crate::customer::Customer;
    pub struct Order{
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }
    
    impl Order{
        fn calculate_discount(&self) -> f64{
            if self.quantity > 5 {
                0.1
            }else{
                0.0
            }
        }
    
        pub fn total_bill(&self) -> f64{
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }

        pub fn new(id: u64 , product: Product , customer: Customer , quantity: u32) -> Order{
            Order { 
                id, 
                product, 
                customer, 
                quantity
            }
        }
    }
}
