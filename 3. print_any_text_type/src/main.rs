fn main() {
    let a = "str";
    let b = "string".to_string();

    info(&a);
    info(&b);
}

fn info<T>(text: &T) 
where
    T: std::fmt::Display,
{
    println!("{}", text);
}
