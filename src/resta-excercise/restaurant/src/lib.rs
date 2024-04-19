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

mod another_front {
    pub mod cook {
        pub fn heat_pan() { println!("b1")}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    use another_front::cook;// can also do use another_front::cook::heat_pan; to call the function directly like heat_pan();

    cook.heat_pan();
}