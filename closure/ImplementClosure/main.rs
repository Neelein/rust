trait Transform<Input>{
    type Output;

    fn transform(self,input:Input) -> Self::Output;
}

fn map<X,Y,T>(option:Option<X>,transform:T) -> Option<Y>
where T:Transform<X,Output = Y>{
    match option{
        Some(x) => Some(transform.transform(x)),
        None => None,
    }
}


struct Adder{x:i32}

impl Transform<i32> for Adder{
    type Output =i32;

    fn transform(self, val:i32) -> i32{
        val + self.x
    }
}

struct Multiplier{y:i32}

impl Transform<i32> for Multiplier{
    type Output = i32;

    fn transform(self, val:i32) -> i32{
        val * self.y
    }
}

fn main(){
    let option = Some(2); 

    let x = 3;
    let new:Option<i32> = map(option,Adder{x:x});
    println!("{:?}",new);

    let y = 10;
    let new2 = map(option,Multiplier{y:y});
    println!("{:?}",new2);
}