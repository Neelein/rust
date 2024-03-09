fn main(){
    let value = [1,2,3,4,5];
    let mut iter = value.iter();

    println!("{:?}",iter.next());
    println!("{:?}",iter.skip(2).take(2).collect::<Vec<_>>());
}