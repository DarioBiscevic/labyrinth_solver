/*
Little note: the given labyrinth is a png file, and luckily it is recognised as
an 8 bit RGB image... PHEW!

Green = start
Red   = finish
*/

mod lib;
mod node;
use lib::*;

use std::env;

fn main() {
    println!("Labyrinth solver");

    //Exit if arguments are invalid; otherwise, proceed
    if let Ok(string) = parse_args(){
        if let Err(()) = run(string){
            println!("Program stopped due to error");
        }
    }else{
        println!("Not enough arguments!!!");
    }
}

fn parse_args() -> Result<String, ()>{
    //Collect and return arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err(())
    }else{
        Ok(args[1].clone())
    }
}