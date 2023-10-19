mod front_of_house;

mod customer {
    use super::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

fn main() {
    customer::eat_at_restaurant();
}
