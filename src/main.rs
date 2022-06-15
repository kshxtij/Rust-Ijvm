pub mod file;

use std::{env::args};
use crate::file::{init_ijvm, print_ijvm};

fn main(){
    let args:Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }
    let ijvm = init_ijvm(args[1].clone());
    print_ijvm(&ijvm);
}

#[cfg(test)]
mod test;