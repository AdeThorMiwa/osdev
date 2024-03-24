use crate::cli::CliContext;
use std::env;

pub fn pwd(_argc: usize, _argv: Vec<String>, _ctx: &mut CliContext) {
    println!("{}", env::current_dir().unwrap().to_str().unwrap())
}
