use crate::common;
use crate::listing;

#[allow(dead_code)]
pub struct Account {
    username: String,
    password: Password,
    bio: String,
    rep: i32,
    birthday: common::Date,
    account_birthday: common::Date,
    listings: Vec<listing::Listing>,
    listings_featured: Vec<listing::Listing>,
}
#[allow(dead_code)]
impl Account {
    fn add_listing(&mut self, listing: listing::Listing) {
        self.listings.push(listing);
    }

    fn add_listing_featured(&mut self, listing: listing::Listing) {
        self.listings_featured.push(listing);
    }
}

#[allow(dead_code)]
struct Password {
    password: String,
}
impl Password {
    #[allow(dead_code)]
    pub fn is_correct(&self, _entry: String) -> bool {
        false
    }
}
