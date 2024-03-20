pub fn match_exsample(){

    let myage:u16 = 36;
    
    match myage{
        35 => {
            println!("Your age is 35");
        }
        156 =>{
            println!("Your age is 156");
        }
        _ =>{
            println!("Your age is 35");
        }
    }
}