use std::{
    env,
    io::{self, stdout, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

//https://github.com/kamalmarhubi/shell-workshop
// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
fn main() {
    'main_loop: loop {
        print!(">>> ");
        let _ = stdout().flush();

        let mut command = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut command).unwrap();

        let input = command.trim();

        if input.is_empty() {
            continue;
        }

        let mut commands = input.split("|").peekable();
        let mut prev_command: Option<std::process::Child> = None;

        while let Some(command) = commands.next() {
            let mut argv = command.split_ascii_whitespace();

            if let Some(program) = argv.next() {
                match program {
                    "cd" => {
                        let new_dir = argv.next().unwrap_or("/");
                        if let Err(e) = env::set_current_dir(PathBuf::from(new_dir)) {
                            eprintln!("{}", e)
                        }
                    }
                    "exit" => break 'main_loop,
                    _ => {
                        let stdin = if let Some(child) = prev_command {
                            Stdio::from(child.stdout.unwrap())
                        } else {
                            Stdio::inherit()
                        };

                        let stdout = if commands.peek().is_some() {
                            Stdio::piped()
                        } else {
                            Stdio::inherit()
                        };

                        match Command::new(program)
                            .args(argv)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn()
                        {
                            Ok(child) => prev_command = Some(child),
                            Err(e) => {
                                prev_command = None;
                                eprintln!("{}", e)
                            }
                        };
                    }
                }
            }
        }

        if let Some(mut last_command) = prev_command {
            let _ = last_command.wait();
        }
    }
}
