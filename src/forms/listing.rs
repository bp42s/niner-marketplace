#[allow(dead_code)]
pub mod listing {
    use crate::{common::category, common::date, item::item};
    pub struct Listing {
        name: String,
        description: String,
        highlight: String,
        item: item::Item,
        price_data: PriceData,
        listing_date_data: ListingDateData,
        category: category::Category,
        hidden: bool,
    }
    impl Listing {
        pub fn is_available(&self) -> bool {
            if self.hidden {
                return false;
            };
            false
        }
    }

    pub struct ListingDateData {
        no_end: bool,
        date_listed: date::Date,
        date_ending: date::Date,
    }
    impl ListingDateData {}

    pub struct PriceData {
        price: f64,
        discount: f64,
        price_negotiable: bool,
    }
    impl PriceData {}
}
