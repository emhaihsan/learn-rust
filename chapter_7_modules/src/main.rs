use crate::garden::vegetables::Asparagus;
// use chapter_7_modules::eat_at_restaurant;
// menggunakan re-exported module dari library crate
use chapter_7_modules::restaurant_hosting;

pub mod garden;

fn main() {
    let plant = Asparagus{};
    println!("I'm growing {plant:?}!");

    restaurant_hosting::add_to_waitlist();

    println!("Eat at restaurant");
}
