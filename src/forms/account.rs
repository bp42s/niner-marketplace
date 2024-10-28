#[allow(dead_code)]
pub mod account {
    use crate::{common::date, common::image, common::keyword, item::item, listing::listing};

    pub struct Account {
        username: String,
        password: Password,
        id: i64,
        bio: String,
        profile_picture: image::Image,
        background_picture: image::Image,
        reputation: Reputation,
        birthday: date::Date,
        account_birthday: date::Date,
        listings_offered: Vec<listing::Listing>,
        listings_featured: Vec<listing::Listing>,
        listings_wanted: Vec<listing::Listing>,
        items_wanted: Vec<item::Item>,
        keywords_wanted: Vec<keyword::Keyword>,
    }
    impl Account {
        fn old_enough(&self) -> bool {
            true
        }

        fn add_listing_offered(&mut self, listing: listing::Listing) {
            self.listings_offered.push(listing);
        }

        fn add_listing_featured(&mut self, listing: listing::Listing) {
            self.listings_featured.push(listing);
        }

        // reputation functions
        pub fn get_rep(&self) -> i32 {
            self.reputation.rep
        }
        pub fn set_rep(&mut self, value: i32) {
            self.reputation.rep = value
        }
        pub fn mod_rep(&mut self, amount: i32) {
            self.reputation.rep = self.reputation.rep + amount
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

    pub struct Reputation {
        rep: i32,
    }
    impl Reputation {}
}
