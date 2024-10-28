#[allow(dead_code)]
pub mod nm_account {
    use crate::{common::nm_date, common::nm_image, common::nm_keyword, item::nm_item, listing::nm_listing};

    pub struct Account {
        username: String,
        password: Password,
        id: u64,
        bio: String,
        profile_picture: nm_image::Image,
        background_picture: nm_image::Image,
        reputation: Reputation,
        user_birthday: nm_date::Date,
        account_birthday: nm_date::Date,
        listings_offered: Vec<nm_listing::Listing>,
        listings_featured: Vec<nm_listing::Listing>,
        listings_wanted: Vec<nm_listing::Listing>,
        items_wanted: Vec<nm_item::Item>,
        keywords_wanted: Vec<nm_keyword::Keyword>,
    }
    impl Account {
        fn old_enough(&self) -> bool {
            true
        }

        fn add_listing_offered(&mut self, listing: nm_listing::Listing) {
            self.listings_offered.push(listing);
        }

        fn add_listing_featured(&mut self, listing: nm_listing::Listing) {
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
            self.reputation.rep += amount
        }
    }

    pub fn create_account() {}

    struct Password {
        password: String,
    }
    impl Password {
        // primitive - needs proper handling
        pub fn is_correct(&self, _entry: String) -> bool {
            false
        }
    }

    pub struct Reputation {
        rep: i32,
    }
    impl Reputation {}
}
