mod front_of_house;

// menggunakan idiomatis: membawa modul induk
use crate::front_of_house::hosting;

// Re-exporting 'hosting' agar bisa diakses langsung
pub use crate::front_of_house::hosting as restaurant_hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    println!("Eat at restaurant");
}
