fn main() {
    let mut users = Vec::new();
    users.push("Todd".to_string());
    users.push("amy".to_string());
    users.push("moni".to_string());
    users.push("munin".to_string());

    let sorted = case_insensitive_sort(&users);

    println!("{:?}", users);
    println!("{:?}", sorted);

}

fn case_insensitive_sort(list: &Vec<String>) -> Vec<String> {
    let mut new_list = list.clone();

    new_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    new_list
}


