use crate::common::{self, category};

#[allow(dead_code)]
pub struct Listing {
    name: String,
    description: String,
    highlight: String,
    price: f64,
    price_negotiable: bool,
    discount: f64,
    listing_dates: ListingDates,
    category: category::Category,
    hidden: bool,
}
#[allow(dead_code)]
impl Listing {
    pub fn is_available(&self) -> bool {
        if self.hidden == true {
            return false;
        };
        false
    }
}

#[allow(dead_code)]
pub struct ListingDates {
    no_end: bool,
    date_listed: common::date::Date,
    date_ending: common::date::Date,
}
impl ListingDates {}
