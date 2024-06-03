use std::fs::read_to_string;

fn main(){
    let content = read_to_string("check.vy").unwrap();
    for c in content.chars(){
        println!("{:?}", c);
    }
}