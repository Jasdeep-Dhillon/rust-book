pub fn take_order() {
    println!("Taking order");
}
pub fn serve_order() {
    self::take_order();
    println!("Serving order");
}
pub fn take_payment() {
    super::hosting::seat_at_table();
    println!("Receiving  payment");
}
