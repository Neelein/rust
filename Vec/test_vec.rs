
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

#[derive(Clone,Debug)]
struct Car{
    manufacturer:String,
    model: String
}

pub fn test_vec_string(){
   let first_names:Vec<&str> =vec!["Trevor","Nancy","Shanono","Billy","Rachel"];

   for name in first_names.as_slice(){
    println!("Processing{}..." ,name);
   }

   println!("{:?}",first_names);
}

pub fn test_vec_car(){
    let mut car_list:Vec<Car> = vec![Car{manufacturer:"Toyota".to_string(),model:"car".to_string()};10];
    for car in &car_list {
        println!("manufacturer:{},model{}",car.manufacturer,car.model);
    }

    let mut car_list2:Vec<Car> = vec![];

    for _ in 1..=100u8{
        car_list2.push(Car{manufacturer:"Toyota".to_string(),model:"car".to_string()});
    }

    car_list.append(&mut car_list2);
    car_list.insert(0,Car{manufacturer:"BMW".to_string(),model:"car".to_string()});

   
    println!("{:?}",car_list.get(101));
}