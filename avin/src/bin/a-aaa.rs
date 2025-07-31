#![allow(dead_code)]
#![allow(unused)]

fn main() {
    println!("Hi");
}

trait Indicator {
    fn trend(n: usize) {
        println!("trend {n}");
    }
}
struct Chart {
    ind: Vec<Box<dyn Indicator>>,
}
