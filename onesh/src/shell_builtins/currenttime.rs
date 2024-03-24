use crate::cli::CliContext;
use std::time::Instant;

pub fn current_time(_argc: usize, _argv: Vec<String>, _ctx: &mut CliContext) {
    println!("current time is: {:?}", Instant::now())
}
