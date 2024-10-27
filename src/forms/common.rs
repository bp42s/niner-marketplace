#[allow(dead_code)]
pub mod date {
    pub struct Date {
        month: i32,
        day: i32,
        year: i32,
    }

    // NOT zero-indexed
    impl Date {
        pub fn get_mdy(&self) -> String {
            let result: String =
                self.month.to_string() + "/" + &self.day.to_string() + "/" + &self.year.to_string();
            result
        }

        pub fn get_dmy(&self) -> String {
            let result: String =
                self.day.to_string() + "/" + &self.month.to_string() + "/" + &self.year.to_string();
            result
        }

        pub fn valid_date(&self) -> bool {
            false
        }

        pub fn get_month_str(&self) -> String {
            match self.month {
                1 => String::from("January"),
                2 => String::from("February"),
                3 => String::from("March"),
                4 => String::from("April"),
                5 => String::from("May"),
                6 => String::from("June"),
                7 => String::from("July"),
                8 => String::from("August"),
                9 => String::from("September"),
                10 => String::from("October"),
                11 => String::from("November"),
                12 => String::from("December"),
                _ => String::from("InvalidMonth"),
            }
        }

        pub fn get_month_max_days(&self) -> i32 {
            match self.month {
                1 => 31,
                2 => {
                    if self.is_leap_year(self.year) {
                        28
                    } else {
                        29
                    }
                }
                3 => 31,
                4 => 30,
                5 => 31,
                6 => 30,
                7 => 31,
                8 => 31,
                9 => 30,
                10 => 31,
                11 => 30,
                12 => 31,
                _ => 0,
            }
        }

        pub fn is_leap_year(&self, year: i32) -> bool {
            if year % 4 != 0 {
                return false;
            }
            if year & 100 != 0 {
                return false;
            }
            if year % 400 != 0 {
                return false;
            }
            true
        }
    }
}

#[allow(dead_code)]
pub mod category {
    pub enum Category {
        ClothingType,
        Decoration,
        Furniture,
        Technology,
    }
    impl Category {}
}

#[allow(dead_code)]
pub mod image {
    pub struct Image {
        path: String,
        caption: String,
        accessibility_text: String,
    }
    impl Image {}
}

#[allow(dead_code)]
pub mod keyword {
    // reusable typing enum - siilar to a "tag"
    pub enum Keyword {
        Whimsical,
        Colorful,
        Dark,
    }
    impl Keyword {}
}
