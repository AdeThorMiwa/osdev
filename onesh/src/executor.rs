use crate::{cli::CliContext, parser::Node, sym_tab::SymTabEntry};
use fork::{fork, Fork};
use std::{
    io::Error,
    os::unix::process::CommandExt,
    path::Path,
    process::{self, Command},
};
use trees::Tree;

pub struct Executor;

impl Executor {
    pub fn search_path(&self, pathname: &str) -> Option<String> {
        let paths = std::env::var("PATH").expect("PATH not defined");
        for path in paths.split(":") {
            let final_path = Path::new(path).join(Path::new(pathname));
            if final_path.exists() {
                if let Some(str) = final_path.to_str() {
                    return Some(str.to_string());
                }
            }
        }

        None
    }

    pub fn exec_command(&self, _argc: usize, argv: Vec<String>) -> usize {
        let bin = &argv[0];

        let program = if bin.contains("/") {
            Some(bin.to_string())
        } else {
            self.search_path(bin)
        };

        if let Some(program) = program {
            Command::new(program).args(argv.iter().skip(1)).exec();
            return 0;
        } else {
            println!("onesh: command not found: {}", argv[0]);
        }

        1
    }

    pub fn run_command(&self, command: Tree<Node>, ctx: &mut CliContext) -> Option<usize> {
        if let Some(_) = command.front() {
            let mut argc = 0;
            let max_args = 255;
            let mut argv = Vec::with_capacity(max_args);

            for child in command.bfs().iter {
                if let Node::Param { value } = child.data {
                    argv.push(value.to_string());
                    argc += 1;
                }
            }

            match fork() {
                Ok(Fork::Parent(child_pid)) => {
                    let exit_code = match nix::sys::wait::waitpid(
                        nix::unistd::Pid::from_raw(child_pid),
                        None,
                    ) {
                        Ok(_) => 0,
                        Err(_) => 1,
                    };

                    return Some(exit_code);
                }
                Ok(Fork::Child) => {
                    if let Some(SymTabEntry::Func { func_body, .. }) =
                        ctx.sym_tab_stack.get_global_sym_tab().get(&argv[0])
                    {
                        func_body(argc, argv, ctx);
                    } else {
                        let _ = self.exec_command(argc, argv);
                    }

                    process::exit(0)
                }
                Err(_) => eprintln!("error: failed to fork command: {}", Error::last_os_error()),
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn test_valid_search_path() {
        let executor = Executor;
        let result = executor.search_path("brew");
        assert!(result.is_some());
        assert_eq!(result, Some("/usr/local/bin/brew".to_string()))
    }

    #[test]
    fn test_invalid_search_path() {
        let executor = Executor;
        let result = executor.search_path("gazurpiazurp");
        assert!(result.is_none());
    }
}
