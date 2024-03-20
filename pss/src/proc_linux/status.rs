use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Debug, Default)]
pub enum ProcessState {
    #[default]
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

#[derive(Debug, Default)]
pub struct Umask(usize);

impl ToString for ProcessState {
    fn to_string(&self) -> String {
        let string_val = match self {
            Self::R => "R (Running)",
            Self::S => "S (Sleeping)",
            Self::D => "D (Uninterruptable Sleep)",
            Self::T => "T (Stopped)",
            Self::Z => "Z (Zombie)",
            Self::I => "I (Idle)",
        };

        string_val.to_string()
    }
}

#[derive(Debug, Default)]
pub struct ProcessStatus {
    pub name: String,
    pub umask: Umask,
    pub state: ProcessState,
}

pub enum ProcessStatusError {
    IOError(io::Error),
    ParseError(String),
}

impl ProcessStatus {
    pub fn new(pid: usize) -> Result<Self, ProcessStatusError> {
        let mut status_file = ProcessStatus::read_process_status_file(pid)?;
        let status = ProcessStatus::parse_file(&mut status_file)?;
        Ok(status)
    }

    fn parse_file(status_file: &mut File) -> Result<Self, ProcessStatusError> {
        let mut buf = String::new();
        let _ = status_file.read_to_string(&mut buf);

        let mut p_status = Self {
            ..Default::default()
        };

        let _: Vec<Result<_, ProcessStatusError>> = buf
            .lines()
            .map(|line| {
                if let Some((key, value)) = line.split_once(":") {
                    let key = key.trim();
                    let value = value.trim();

                    match key {
                        "Name" => {
                            p_status.name = value.to_string();
                        }
                        "Umask" => {
                            let umask = value.parse::<usize>().map_err(|_| {
                                ProcessStatusError::ParseError("invalid umask".to_string())
                            })?;
                            p_status.umask = Umask(umask)
                        }
                        "State" => {
                            let state = match value {
                                v if v.starts_with("I") => ProcessState::I,
                                v if v.starts_with("R") => ProcessState::R,
                                v if v.starts_with("S") => ProcessState::S,
                                v if v.starts_with("D") => ProcessState::D,
                                _ => {
                                    println!("default status {} to zombie", value);
                                    ProcessState::Z
                                }
                            };
                            p_status.state = state;
                        }
                        _ => {}
                    }
                }

                Ok(())
            })
            .collect();

        Ok(p_status)
    }

    fn read_process_status_file(pid: usize) -> Result<File, ProcessStatusError> {
        match File::open(format!("/proc/{}/status", pid)) {
            Ok(file) => Ok(file),
            Err(e) => Err(ProcessStatusError::IOError(e)),
        }
    }
}
