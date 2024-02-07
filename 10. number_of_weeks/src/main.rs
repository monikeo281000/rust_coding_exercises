pub mod My_date;

fn main() {
    let mut date1 = String::from("2024-2-13");
    let mut date2 = String::from("2024-2-6");

    let weeks = My_date::weeks_between(&date1, &date2);

    println!("weeks = {}", weeks);
}
