fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // K_22710 这里supper意思为调用mod上级模块的成员
    }

    fn cook_order() {}
}
