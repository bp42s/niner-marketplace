pub struct Date {
    month: i32,
    day: i32,
    year: i32
}

impl Date {
    pub fn get_mdy(&self) -> String {
        self.month.to_string() + "/" + &self.day.to_string() + "/" + &self.year.to_string()
    }

    pub fn get_dmy(&self) -> String {
        self.day.to_string() + "/" + &self.month.to_string() + "/" + &self.year.to_string()
    }
}