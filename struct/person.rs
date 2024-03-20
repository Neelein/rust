struct Person<PetType> where PetType:Animal
{
    first_name:String,
    last_name:String,
    pet:PetType
}


trait Animal{
    fn make_sound(&self) -> ();
}

struct Cat{

}
impl Animal for Cat{
    fn make_sound(&self){
        println!("cat mewo!");
    }
}

pub fn create_person(){
    let pet = Cat{};
    let p1 = Person{first_name:"neal".to_string(),last_name:"sha".to_string(),pet};
    p1.pet.make_sound();
}
