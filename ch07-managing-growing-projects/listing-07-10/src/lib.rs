mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use back_of_house::Appetizer;

pub fn eat_at_restaurant() {
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}
