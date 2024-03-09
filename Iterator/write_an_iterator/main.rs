struct CountUp{
    current:usize,
}

impl Iterator for CountUp{
    type Item =usize;
    fn next(&mut self) -> Option<usize>{
        self.current +=1;
        Some(self.current)
    }
}

fn main(){
    let iterator = CountUp{current:0};
    let output = iterator.take(20).collect::<Vec<_>>();
    println!("{:?}",output);
}