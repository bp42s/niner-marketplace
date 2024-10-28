#[allow(dead_code)]
pub mod nm_listing {
    use crate::{nm_common::{nm_date, nm_image, nm_keyword, nm_tags}, nm_item::nm_item};
    pub struct Listing {
        name: String,
        description: String,
        highlight: String,
        image_thumbnail: nm_image::Image,
        image_showcase: Vec<nm_image::Image>,
        hidden: bool,
        item: nm_item::Item,
        price_data: PriceData,
        listing_date_data: ListingDateData,
        keywords: Vec<nm_keyword::Keyword>,
        tags: nm_tags::Tags,
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
        date_listed: nm_date::Date,
        date_ending: nm_date::Date,
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
            self.price -= amount;
            amount
        }
    }
}
