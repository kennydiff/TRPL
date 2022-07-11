mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;  // K_22710 这里use引入，要放到mod customer里，否则scope不对，会报未声明的错误
mod customer {
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
