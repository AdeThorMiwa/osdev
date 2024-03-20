use std::process;

use crate::cli::Cli;

pub struct REPL;

pub enum PromptLevel {
    PS1,
    PS2,
}

impl REPL {
    pub fn init() {
        let cli = Cli::new();

        loop {
            Self::prompt(PromptLevel::PS1);

            let command = cli.read_cmd();

            if command.as_bytes()[0] == b'\0' || command == "\n" {
                continue;
            }

            if command == "exit\n" {
                break;
            }

            print!("{}", command)
        }

        process::exit(0)
    }

    pub fn prompt(level: PromptLevel) {
        match level {
            PromptLevel::PS1 => eprint!("$ "),
            PromptLevel::PS2 => eprint!("> "),
        }
    }
}
