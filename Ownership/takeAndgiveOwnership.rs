pub fn take_and_give(){
    let s = give_ownership();
    let s2 = String::from("hello");
    let s3 = take_and_give_back(s2);
    println!("{} {}",s,s3);
}

fn give_ownership() -> String{
    let some_string = String::from("hello");
    some_string
}

fn take_and_give_back(a_string:String) -> String{
    a_string
}