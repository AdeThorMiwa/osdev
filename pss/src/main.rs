use clap::Parser;

const LONG_ABOUT: &'static str = "ps  gives a snapshot of the current processes. If you want a repetitive update of this status, use top. This man page documents the /proc-based version of ps, or tries to.";

#[derive(Parser, Debug)]
#[command(version, about = "pss - report process status", long_about = LONG_ABOUT)]
struct Options {
    #[arg(short = 'e', help = "Select all processes")]
    all: bool,

    #[arg(short = 'a', help = "select all with a tty except session leaders")]
    all_with_tty: bool,

    #[arg(short = 't', help = "select all processes on this terminal")]
    all_terminal_process: bool,

    #[arg(short = 'x', help = "select processes without controlling ttys")]
    all_without_contr_tty: bool,

    #[arg(short = 'c', help = "select processes by command")]
    command: Option<String>,

    #[arg(long = "pid", help = "select processes by process id")]
    pid: Option<usize>,

    #[arg(long = "ppid", help = "select processes by process parent id")]
    ppid: Option<usize>,

    #[arg(long = "rgid", help = "select processes by process rgid")]
    rgid: Option<usize>,

    #[arg(long = "ruid", help = "select processes by process ruid")]
    ruid: Option<usize>,
}

#[cfg(target_os = "macos")]
fn main() {
    let _ = Options::parse();

    eprintln!("not supported")
}

#[cfg(target_os = "linux")]
fn main() -> anyhow::Result<()> {
    use pss::proc_linux::status::ProcessStatus;
    use std::fs::read_dir;
    use term_table::{row::Row, table_cell::TableCell, Table, TableStyle};

    let options = Options::parse();

    let mut table = Table::new();
    table.style = TableStyle::blank();

    if options.all {
        table.add_row(Row::new(vec![
            TableCell::new("PID"),
            TableCell::new("CMD"),
            TableCell::new("Status"),
        ]));

        for entry in read_dir("/proc")? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Ok(pid) = path.file_name().unwrap().to_str().unwrap().parse::<usize>() {
                    if let Ok(process_status) = ProcessStatus::new(pid) {
                        table.add_row(Row::new(vec![
                            TableCell::new(pid),
                            TableCell::new(process_status.name),
                            TableCell::new(process_status.state),
                        ]))
                    } else {
                        eprintln!("process with pid: {} is invalid", pid)
                    }
                }
            }
        }
    }

    println!("{}", table.render());

    Ok(())
}
