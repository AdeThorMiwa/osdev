use onesh::cli::Cli;

fn main() -> std::io::Result<()> {
    let mut cli = Cli::new();
    cli.repl()
}
