#[allow(dead_code)]
pub mod account {
    use crate::common::{self};
    use crate::listing::listing;

    pub struct Account {
        username: String,
        password: Password,
        id: i64,
        bio: String,
        rep: i32,
        birthday: common::date::Date,
        account_birthday: common::date::Date,
        listings: Vec<listing::Listing>,
        listings_featured: Vec<listing::Listing>,
    }
    impl Account {
        fn is_old_enough(&self) -> bool {
            false
        }
        fn add_listing(&mut self, listing: listing::Listing) {
            self.listings.push(listing);
        }

        fn add_listing_featured(&mut self, listing: listing::Listing) {
            self.listings_featured.push(listing);
        }
    }

    pub fn create_account() {}

    struct Password {
        password: String,
    }
    impl Password {
        pub fn is_correct(&self, _entry: String) -> bool {
            false
        }
    }
}
