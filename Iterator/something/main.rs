use std::collections::VecDeque;

fn main(){
    let input = [ 1,2,3,];
    let iterator = input.iter();
    let mapped = iterator.map(|&x|{
        return x * 2;
    });
    let output = mapped.collect::<VecDeque<_>>();
    println!("{:?}",output);
}