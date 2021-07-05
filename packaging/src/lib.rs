mod hotel {
    pub mod room {
        pub fn check_available_room() {}
        pub fn book_room() {}
        pub fn unbook_room() {}
    }
    mod restaurant {
        fn take_order() {}
        fn pay_bill() {}
    }
}

pub fn get_room() {
    crate::hotel::room::check_available_room();
    hotel::room::book_room();
}

use crate::hotel::room;

pub fn remove_room(){
    room::unbook_room()
    room::check_available_room()
}

pub use hotel;