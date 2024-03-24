use crate::{
    cli::CliContext,
    sym_tab::{SymTab, SymTabEntry},
};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum StackKind {
    Global,
    Local,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, value_enum, default_value_t = StackKind::Local)]
    sym_tab: StackKind,
}

fn dump_sym_tab(sym_tab: &mut SymTab) {
    let mut i = 0;
    for (.., entry) in sym_tab.get_entries() {
        if let SymTabEntry::Str { symbol, value, .. } = entry {
            println!("[{:0>4}]  {}={}", i, symbol, value);
            i += 1;
        }
    }
}

fn dump_local(ctx: &mut CliContext) {
    let local_sym_tab = ctx.sym_tab_stack.get_local_sym_tab_mut();
    dump_sym_tab(local_sym_tab)
}

fn dump_global(ctx: &mut CliContext) {
    let global_sym_tab = ctx.sym_tab_stack.get_global_sym_tab_mut();
    dump_sym_tab(global_sym_tab)
}

pub fn dump(_argc: usize, argv: Vec<String>, ctx: &mut CliContext) {
    let args: Args = Args::parse_from(argv);

    match args.sym_tab {
        StackKind::Global => dump_global(ctx),
        StackKind::Local => dump_local(ctx),
    }
}
