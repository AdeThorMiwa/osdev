use crate::{executor::Executor, parser::Parser, tokenizer::Tokenizer};
use std::process;

pub enum PromptLevel {
    PS1,
    PS2,
}

pub struct Cli {
    executor: Executor,
    parser: Parser,
}

impl Cli {
    pub fn new() -> Self {
        let executor = Executor;
        let parser = Parser;
        Self { executor, parser }
    }

    pub fn repl(&mut self) -> std::io::Result<()> {
        loop {
            self.prompt(PromptLevel::PS1);

            let command = self.read_cmd();

            if command.as_bytes()[0] == b'\0' || command == "\n" {
                continue;
            }

            if command == "exit\n" {
                break;
            }

            self.parse_and_execute(command);
        }

        process::exit(0)
    }

    fn parse_and_execute(&mut self, command: String) {
        let mut tokenizer = Tokenizer::new(&command);
        tokenizer.skip_whitespaces();
        while let Some(token) = tokenizer.tokenize() {
            let command = self.parser.parse(&token, &mut tokenizer);
            let _ = self.executor.run_command(command);
        }
    }

    fn read_cmd(&self) -> String {
        let mut cmd = String::new();
        let mut buf = String::new();

        while let Ok(buflen) = std::io::stdin().read_line(&mut buf) {
            if buflen > 0 && buf.chars().nth(buflen - 1) == Some('\n') {
                if buflen == 1 || buf.chars().nth(buflen - 2) != Some('\\') {
                    cmd.push_str(&buf);
                    return cmd;
                }

                cmd.push_str(&buf[..buf.len() - 2]);
                buf.clear();
                self.prompt(PromptLevel::PS2)
            }
        }

        cmd
    }

    pub fn prompt(&self, level: PromptLevel) {
        match level {
            PromptLevel::PS1 => eprint!("$ "),
            PromptLevel::PS2 => eprint!("> "),
        }
    }
}
