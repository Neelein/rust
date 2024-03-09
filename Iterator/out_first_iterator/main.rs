using 

fn main(){
    let input = [1,2,3];

    let iterator = input.iter();
    
    let mapped = iterator.map(|&x| x * 2);

    let output = mapped.collect::<Vec<usize>>();
    
    println!("{:?}",output);
}