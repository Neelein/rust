pub fn exsample1(){
    let x = 5;
    let y = x; //copy

    let s1 =String::from("hello");
    //let s2 = s1; //Move(not shallow copy)

    let s2 = s1.clone();
    println!("{}",s1);
}