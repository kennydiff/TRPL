mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;  // K_22710 这个crate use 要和引用的地方一致, 在一个Scope内

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
