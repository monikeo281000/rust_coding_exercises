use rand::Rng;

fn main() {
    let mut list_number: Vec<f32> = Vec::new();

    let mut rng = rand::thread_rng();

    for i in 0..10 {
        list_number.push(rng.gen_range(0.0..100.0));
    }

    println!("{:?}", list_number);

    //list_number.sort_by(|x, y| x.partial_cmp(y).unwrap());

    println!("{:?}", list_number); 

    let median_value = median(&mut list_number);
    println!("{:?}", median_value);
}

fn sort_vec(list_number: &mut Vec<f32>) {
    if list_number.len() < 1 {
       return; 
    }

    for i in 0..list_number.len() {
        for j in 0..list_number.len() {
            if list_number[i] < list_number[j] {
                let temp = list_number[i];
                list_number[i] = list_number[j];
                list_number[j] = temp;
            }
        }
    }
}

fn median(sort_list: &mut Vec<f32>) -> Option<f32> {
    sort_list.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let size = sort_list.len();
    
    if sort_list.is_empty() {
        return None;
    } else if size % 2 != 0 {
        Some(sort_list[size/2])
    } else {
        Some((sort_list[size/2] + sort_list[size/2 - 1]) / 2.0)
    }

}

