
pub mod deadline;
use deadline::{Deadline, ImportantEvent}; 

fn main() {
    test01();
}

fn test01() {
    let event_name = "Open Hourse".to_string();
    let year = 2024;
    let month = 2;
    let day = 8;
    let event = ImportantEvent::new(event_name, year, month, day);

    if event.is_passed() {
        println!("Expired");
    } else {
        println!("Comming Soon");
    }
}
