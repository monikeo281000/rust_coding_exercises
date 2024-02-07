pub mod MissingValue;

fn main() {
    let mut lists: Vec<Option<i32>> = Vec::new();

    lists.push(Some(2));
    lists.push(None);
    lists.push(Some(4));
    lists.push(None);

    let sum = MissingValue::sum_with_missing(&lists);

    println!("sum = {}", sum);
}
