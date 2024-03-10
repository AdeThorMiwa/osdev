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
    pid: Option<u8>,

    #[arg(long = "ppid", help = "select processes by process parent id")]
    ppid: Option<u8>,

    #[arg(long = "rgid", help = "select processes by process rgid")]
    rgid: Option<u8>,

    #[arg(long = "ruid", help = "select processes by process ruid")]
    ruid: Option<u8>,
}

#[cfg(target_os = "macos")]
fn main() {
    let _ = Options::parse();

    println!("this is mac os main func")
}

#[cfg(target_os = "linux")]
fn main() {
    let _ = Options::parse();

    println!("this is linux os main func")
}
