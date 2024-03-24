use onesh::{cli::Cli, shell_builtins::builtins};

fn main() {
    let builtin_list = builtins();
    let builtin_list = builtin_list.iter().collect();
    let mut cli = Cli::new(builtin_list);
    let _ = cli.init();
}
