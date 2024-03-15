struct Person{
    first_name:String,
    last_name:String
}


fn main(){
    let x = 30;
    let mut person = Person{first_name:"Json".to_string(),last_name:"Terry".to_string()};

    let add = |y| x+y;
    let result = add(10);
    println!("the result is {}",result);

    let mut change = ||{
        person.first_name = "sha".to_string()
    };

    change();

    println!("the first name change {}",person.first_name);
}