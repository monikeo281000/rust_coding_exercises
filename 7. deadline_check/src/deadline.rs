use chrono;

type Year = i32;
type Month = u32;
type Day = u32;

pub trait Deadline {
    fn is_passed(&self) -> bool;
}

pub struct ImportantEvent {
    event_name: String,
    date: chrono::NaiveDate,
}

impl ImportantEvent {
    pub fn new(event_name:String, year:Year, month:Month, day:Day) -> Self {
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
        Self{
            event_name,
            date,
        }
    }

    pub fn event_name(&self) -> &str {
        &self.event_name
    }
    
    pub fn date(&self) -> &chrono::NaiveDate {
        &self.date
    }
}


impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        let mut flag = true;
        let local_now = chrono::Local::now().date_naive();
        let date_remainder = local_now.signed_duration_since(self.date).num_days();
        println!("{}", date_remainder);
        if date_remainder <= 0 {
            flag = false;
        }
        flag
    }
}
