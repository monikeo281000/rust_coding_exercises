pub fn sum_with_missing(lists: &Vec<Option<i32>>) -> i32 {
    let mut sum = 0 ;
    let sum = lists.iter()
        .map(|&x| x.unwrap_or(0))
        .sum();
    sum
}
