// #![allow(dead_code, unused_imports)]

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    let mut i=vec.len()-1;
    while i>0{
        i-=1;
        println!("{}",vec[i]);
    }
}
