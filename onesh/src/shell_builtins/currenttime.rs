use std::time::Instant;

pub fn current_time(_argc: usize, _argv: Vec<String>) {
    eprintln!("current time is: {:?}", Instant::now())
}
