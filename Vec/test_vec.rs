
#[allow(dead_code)]
pub  fn test_vec_int(){
     let mut my_ints:Vec<i32> = Vec::new();

    my_ints.push(30);
    my_ints.push(40);
    my_ints.push(30);
    my_ints.push(40);
    my_ints.push(30);
    my_ints.push(40);
    
    println!("{:?}",&(&my_ints).as_slice()[0..5]);

    // if the element is not in the vec the get methond will return none
    println!("First element is {:?}",my_ints.get(10));
}

pub fn test_vec_string(){
   let first_names:Vec<&str> =vec!["Trevor","Nancy","Shanono","Billy","Rachel"];

   for name in first_names.as_slice(){
    println!("Processing{}..." ,name);
   }

   println!("{:?}",first_names);
}