pub mod shop {
    pub fn take_item(){}
}

pub mod buy {
    pub mod refund {
        pub fn request() {}
    }
    pub mod claims {
        pub fn request() {}
    }
}

fn deliver_order() {}

use crate::shop::{take_item};

mod employee {
    // use crate::shop;

    use crate::shop::take_item;

    fn deliver() {
        super::deliver_order();
    }

    fn take_item_customer() {
        take_item();
    }
}