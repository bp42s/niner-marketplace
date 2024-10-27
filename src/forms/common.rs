pub struct Date {
    month: i32,
    day: i32,
    year: i32,
}

// NOT zero-indexed
#[allow(dead_code)]
impl Date {
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

    pub fn get_mdy(&self) -> String {
        self.month.to_string() + "/" + &self.day.to_string() + "/" + &self.year.to_string()
    }

    pub fn get_dmy(&self) -> String {
        self.day.to_string() + "/" + &self.month.to_string() + "/" + &self.year.to_string()
    }
}
