pub fn OwnerShip(){
    let s = String::from("hello");
    //take_ownership(s); // take ownership to the function not right
    take_ownership(s.clone());//use clone is correct
    println!("{}",s);

    let x = 5;
    make_copy(x);
    println!("{}",x);
}

fn take_ownership(some_string:String){
    println!("{}",some_string);
}

fn make_copy(some_interger:i32){
    println!("{}",some_interger);
}