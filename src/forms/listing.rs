#[allow(dead_code)]
pub mod listing {
    use crate::{common::category, common::date, common::image, common::keyword, item::item};
    pub struct Listing {
        name: String,
        description: String,
        highlight: String,
        image_thumbnail: image::Image,
        image_showcase: Vec<image::Image>,
        hidden: bool,
        item: item::Item,
        price_data: PriceData,
        listing_date_data: ListingDateData,
        category: category::Category,
        keywords: Vec<keyword::Keyword>,
    }
    impl Listing {
        pub fn is_available(&self) -> bool {
            if self.hidden {
                return false;
            };
            true
        }
    }

    pub struct ListingDateData {
        no_end: bool,
        date_listed: date::Date,
        date_ending: date::Date,
    }
    impl ListingDateData {
        pub fn get_available(&self) -> bool {
            true
        }
    }

    pub struct PriceData {
        price: f64,
        discount: f64,
        price_negotiable: bool,
    }
    impl PriceData {
        pub fn price_with_discount(&self) -> f64 {
            let result: f64 = self.price * self.discount;
            result
        }
    }
}
