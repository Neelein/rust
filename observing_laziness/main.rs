fn main(){
    let input = [1,2,3];
    let iterator = input.iter();
    let mappind = iterator
    .inspect(|&x| println!("Pre map:\t{}",x))
    .map(|&x| x*10)
    .inspect(|&x| println!("first map:\t{}",x))
    .map(|x| x + 5)
    .inspect(|&x| println!("second map:\t{}",x));

    mappind.collect::<Vec<usize>>();
}