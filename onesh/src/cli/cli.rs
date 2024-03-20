use crate::repl::{PromptLevel, REPL};

pub struct Cli;

impl Cli {
    pub fn new() -> Self {
        Self
    }

    pub fn read_cmd(&self) -> String {
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
                REPL::prompt(PromptLevel::PS2)
            }
        }

        cmd
    }
}
