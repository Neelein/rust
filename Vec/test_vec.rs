pub  fn test_vec_int(){
     let mut my_ints:Vec<i32> = Vec::new();

    my_ints.push(30);
    my_ints.push(40);
    my_ints.push(30);
    my_ints.push(40);
    my_ints.push(30);
    my_ints.push(40);
    
    println!("{:?}",&(&my_ints).as_slice()[0..5]);
}