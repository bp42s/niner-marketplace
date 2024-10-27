use crate::common;

#[allow(dead_code)]
pub struct Listing {
    name: String,
    description: String,
    highlight: String,
    price: f64,
    date_listed: common::Date,
    date_end: common::Date,
    category: Category,
}
impl Listing {}

#[allow(dead_code)]
struct Category {}
impl Category {}
