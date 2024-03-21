use fork::{fork, Fork};
use std::{io::Error, os::unix::process::CommandExt, path::Path, process::Command};
use trees::Tree;

use crate::parser::Node;

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

    pub fn exec_command(&self, _argc: usize, argv: Vec<String>) -> Result<(), usize> {
        let bin = &argv[0];

        let program = if bin.contains("/") {
            Some(bin.to_string())
        } else {
            self.search_path(bin)
        };

        if let Some(program) = program {
            Command::new(program).args(argv.iter().skip(1)).exec();
            return Ok(());
        }

        Err(0)
    }

    pub fn run_command(&self, root: Tree<Node>) -> Result<(), usize> {
        if let Some(_) = root.front() {
            let mut argc = 0;
            let max_args = 255;
            let mut argv = Vec::with_capacity(max_args);

            for child in root.bfs().iter {
                if let Node::Param { value } = child.data {
                    argv.push(value.to_string());
                    argc += 1;
                }
            }

            match fork() {
                Ok(Fork::Parent(child_pid)) => {
                    match nix::sys::wait::waitpid(nix::unistd::Pid::from_raw(child_pid), None) {
                        Ok(_) => {}
                        Err(_) => eprintln!("Error waiting"),
                    }
                }
                Ok(Fork::Child) => {
                    let _ = self.exec_command(argc, argv);
                }
                Err(_) => eprintln!("error: failed to fork command: {}", Error::last_os_error()),
            }
        }

        Err(0)
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
