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
    listing_featured: Vec<listing::Listing>,
}
impl Account {}

struct Password {}
impl Password {}
