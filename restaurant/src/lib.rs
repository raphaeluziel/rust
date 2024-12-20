#![allow(warnings)]

use front_of_house::hosting;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast", meal.toast);

    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}


