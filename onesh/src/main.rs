use onesh::{cli::Cli, shell_builtins::builtins};

fn main() -> std::io::Result<()> {
    let builtin_list = builtins();
    let builtin_list = builtin_list.iter().collect();
    let mut cli = Cli::new(builtin_list);
    cli.init()
}
