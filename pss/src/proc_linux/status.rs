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

impl TryFrom<&str> for ProcessState {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let state = match value {
            _ => ProcessState::R,
        };
        println!("process state >> {}", value);

        Ok(state)
    }
}

pub struct ProcessStatus {
    pub name: String,
    pub umask: usize,
    pub state: ProcessState,
}

impl ProcessStatus {
    pub fn new(pid: usize) -> Result<Self, ()> {
        let mut status_file =
            File::open(format!("/proc/{}/status", pid)).expect("Invalid process ID");
        let mut buf = String::new();
        let _ = status_file.read_to_string(&mut buf);
        let parsed = buf
            .split("\n")
            .map(|line| {
                let mapped = line.split(":");
                mapped.collect()
            })
            .collect::<Vec<Vec<&str>>>();

        let name = parsed.get(0).unwrap().get(1).unwrap();
        let state = parsed.get(2).unwrap().get(1).unwrap();
        let state = match ProcessState::try_from(*state) {
            Ok(state) => state,
            Err(_) => return Err(()),
        };

        Ok(Self {
            name: name.to_string(),
            state: state,
            umask: 23,
        })
    }
}
