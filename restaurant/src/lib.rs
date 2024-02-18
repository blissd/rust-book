mod front_of_house;

pub use front_of_house::hosting;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    // Order breakfast with summer Rye toast.
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");

    // Change our mind about the bread we want.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
