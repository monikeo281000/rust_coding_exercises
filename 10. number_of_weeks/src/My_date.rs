use chrono::NaiveDate;


pub fn weeks_between(date1: &str, date2: &str) -> i64 {
    let date1 = NaiveDate::parse_from_str(date1, "%Y-%m-%d");
    let date2 = NaiveDate::parse_from_str(date2, "%Y-%m-%d");
    let mut result = 0;

    if date1.is_err() || date2.is_err() {
        result = -1;
    } else {
        let days = date1.unwrap().signed_duration_since(date2.unwrap());
        result = days.num_weeks();
    }

    result
}
