pub fn calculate(){
    let s1 = String::from("hello");
    let len:usize =calculate_length(&s1);
    println!("The length is {}",len);
}

fn calculate_length(s:&String) -> usize{
    let length:usize = s.len();
    length
}