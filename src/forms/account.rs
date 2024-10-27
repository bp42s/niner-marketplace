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
impl Account {}

#[allow(dead_code)]
struct Password {
    password: String,
}
impl Password {
    pub fn is_correct(&self, entry: String) -> bool {
        false
    }
}
