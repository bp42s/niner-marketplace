#[allow(dead_code)]
pub mod listing {
    use crate::{common::date, common::image, common::keyword, common::tags, item::item};
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
        keywords: Vec<keyword::Keyword>,
        tags: tags::Tags,
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
            if self.no_end {
                return true;
            }
            true
        }
    }

    pub struct PriceData {
        price: f64,
        discount: f64,
    }
    impl PriceData {
        pub fn price_with_discount(&self) -> f64 {
            let result: f64 = self.price * self.discount;
            result
        }

        pub fn take_fee(&mut self, amount: f64) -> f64 {
            self.price = self.price - amount;
            amount
        }
    }
}
