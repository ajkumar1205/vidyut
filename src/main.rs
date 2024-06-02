use std::{env, process::exit};
use std::result::Result;

mod lex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 2 {
        println!("Too many arguments");
        println!("Usage vid <path>/file.vy");
        exit(1);
    }
    if args.len() < 2 {
        help();
        exit(1);
    }

    let file = &args[1];

    let content = std::fs::read_to_string(file)?;

    println!("Content: \n{}", content);

    Ok(())
}

fn help(){
    println!("Help is here");
}