
use std::collections::HashSet;

fn main() {
    let mut my_list = vec![1,4,5,2,1,2,2,6];

    println!("{:?}", unique_items(&my_list));

    println!("{:?}", unique(&mut my_list));
}

fn empty_list<T>(items: &Vec<T>) -> bool {
    if items.is_empty() {
        return true;
    }
    false
}

fn unique_items<T>(items: &Vec<T>) -> Vec<T> 
where
    T: PartialEq + std::hash::Hash + std::cmp::Eq + Clone,
{
    let mut new_list: Vec<T> = Vec::new();

    if empty_list(&items) {
        return new_list;
    }

    let mut seen = HashSet::new();

    for item in items {
        if seen.insert(item) {
            new_list.push(item.clone());
        }
    }
    new_list
}

fn unique<T>(items: &mut Vec<T>) -> Vec<T> 
where
    T: PartialOrd + Ord + Clone,
{
    items.sort();
    items.dedup();
    items.to_vec()
}
