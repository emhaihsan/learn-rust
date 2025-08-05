pub mod hosting {
    pub fn add_to_waitlist() {} // ini jadi public

     fn seat_at_table(){} // ini privat
}

mod serving { // ini tetap private
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
}
