/**
 * Modules: organize & handle privacy (which variables/fn/...etc can be viewed/called by other codes)
 * Crates: modules that produce a library or executable
 * Packages: build, test, and share crates
 * Paths: a way of naming an item such as a struct, function
 *
 */

mod pizza_order {

    // To access the struct and the part to make public must both use pub
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    // Implement functionality for the Pizza struct
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("regular dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }

    // help_customer is public so functions can call it
    pub mod help_customer {
        // This function is private
        fn seat_at_table() {
            println!("Customer seated at table");
        }

        // Making help_customer public doesn't make this child
        // function public so we must also make it public
        pub fn take_order() {
            seat_at_table();

            // super allows me to access pizza in the parent scope
            let cust_pizza: super::Pizza = super::Pizza::lunch("veggies");

            serve_customer(cust_pizza);
        }

        fn serve_customer(cust_pizza: super::Pizza) {
            println!(
                "The customer is served a regular pizza with {}",
                cust_pizza.topping
            );
        }
    }
}

// This is the public function that allows our other file access
pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
