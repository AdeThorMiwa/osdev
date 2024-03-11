use std::{fs::File, io::Read};

#[derive(Debug)]
pub enum ProcessState {
    /// Running or Runnable
    R,
    /// Interruptable Sleep
    S,
    /// Uninterruptable Sleep
    D,
    /// Stopped
    T,
    /// Zombie
    Z,
    /// Idle
    I,
}

impl ToString for ProcessState {
    fn to_string(&self) -> String {
        "Running".to_string()
    }
}

pub struct ProcessStatus {
    pub name: String,
    pub umask: usize,
    pub state: ProcessState,
}

impl ProcessStatus {
    pub fn new(pid: usize) -> Self {
        let mut status_file =
            File::open(format!("/proc/{}/status", pid)).expect("Invalid process ID");
        let mut buf = String::new();
        let _ = status_file.read_to_string(&mut buf);
        let parsed = buf
            .split("\n")
            .map(|line| {
                let mapped = line.split(":").map(|n| n.to_string());
                mapped.collect()
            })
            .collect::<Vec<Vec<String>>>();

        let name = parsed.get(0).unwrap().get(1).unwrap();
        Self {
            name: name.to_string(),
            state: ProcessState::R,
            umask: 23,
        }
    }
}
