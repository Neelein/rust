pub fn rule(){
    let mut s = String::from("hello");

    let r1 = &s;  //use the inmutalbe reference is ok
    let r2 = &s;

    //let r3 = &mut s; //if you had used the inmutalbe reference use can't use the murable refrence 

    println!("{} {}",r1,r2);

    let r3 = &mut s; // this is correct because the r1 r2 be borrowed 
}