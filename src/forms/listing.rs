use crate::common;

#[allow(dead_code)]
pub struct Listing {
    name: String,
    description: String,
    highlight: String,
    price: f64,
    price_negotiable: bool,
    discount: f64,
    date_listed: common::Date,
    date_end: common::Date,
    category: Category,
}
#[allow(dead_code)]
impl Listing {
    pub fn is_available() -> bool {
        false
    }
}

#[allow(dead_code)]
struct Category {}
impl Category {}
