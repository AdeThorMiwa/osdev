use crate::{
    builtin::{BuiltIn, BuiltInList},
    executor::Executor,
    parser::Parser,
    sym_tab::{SymTabEntry, SymTabStack},
    tokenizer::Tokenizer,
};
use std::{env, process};

pub enum PromptLevel {
    PS1,
    PS2,
}

pub struct CliContext<'a> {
    pub sym_tab_stack: &'a mut SymTabStack,
}

pub struct Cli<'a> {
    executor: Executor,
    parser: Parser,
    pub(crate) sym_tab_stack: SymTabStack,
    builtin_list: BuiltInList<'a>,
}

impl<'a> Cli<'a> {
    pub fn new(builtins: Vec<&'a BuiltIn>) -> Self {
        let executor = Executor;
        let parser = Parser;
        let sym_tab_stack = SymTabStack::new();
        let builtin_list: BuiltInList<'a> = BuiltInList::from(builtins);

        Self {
            executor,
            parser,
            sym_tab_stack,
            builtin_list,
        }
    }

    pub fn init(&mut self) {
        self.init_global_symbol_table();
        self.load_shell_builtins();
        self.repl()
    }

    pub fn load_shell_builtins(&mut self) {
        let global_sym_tab = self.sym_tab_stack.get_global_sym_tab_mut();

        for builtin in &mut self.builtin_list {
            let symbol = builtin.name.clone();
            let sym_tab_entry = SymTabEntry::Func {
                symbol: symbol.clone(),
                func_body: builtin.func,
            };

            global_sym_tab.insert(&symbol, sym_tab_entry);
        }
    }

    fn init_global_symbol_table(&mut self) {
        self.load_env_sym_tab_entries();
        self.load_prompt_sym_tab_entries()
    }

    fn load_env_sym_tab_entries(&mut self) {
        let global_sym_tab = self.sym_tab_stack.get_global_sym_tab_mut();

        for (var_name, var_value) in env::vars() {
            let sym_entry = SymTabEntry::Str {
                symbol: var_name.clone(),
                flags: 0,
                value: var_value,
            };

            global_sym_tab.insert(&var_name, sym_entry);
        }
    }

    fn load_prompt_sym_tab_entries(&mut self) {
        let ps1 = "PS1";
        let ps2 = "PS2";
        let ps1_entry = SymTabEntry::new_str(ps1, "$ ");
        let ps2_entry = SymTabEntry::new_str(ps2, "?> ");

        let global_symbol_tab = self.sym_tab_stack.get_global_sym_tab_mut();
        global_symbol_tab.insert(ps1, ps1_entry);
        global_symbol_tab.insert(ps2, ps2_entry);
    }

    fn repl(&mut self) {
        loop {
            self.prompt(PromptLevel::PS1);

            let command = self.read_cmd();

            match command {
                c if c == "exit\n" => process::exit(0),
                c if c.len() > 0 && (c.as_bytes()[0] == b'\0' || c == "\n") => continue,
                _ => {
                    let _ = self.parse_and_execute(command);
                }
            }
        }
    }

    fn parse_and_execute(&mut self, command: String) {
        let mut tokenizer = Tokenizer::new(&command);
        tokenizer.skip_whitespaces();

        let mut ctx = CliContext {
            sym_tab_stack: &mut self.sym_tab_stack,
        };

        while let Some(token) = tokenizer.tokenize() {
            let command = self.parser.parse(&token, &mut tokenizer);
            let _ = self.executor.run_command(command, &mut ctx);
        }
    }

    fn read_cmd(&self) -> String {
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
                self.prompt(PromptLevel::PS2)
            }
        }

        cmd
    }

    fn prompt(&self, level: PromptLevel) {
        match level {
            PromptLevel::PS1 => {
                let prompt_symbol = match self.sym_tab_stack.get_global_sym_tab().get("PS1") {
                    Some(SymTabEntry::Str { value, .. }) => value,
                    _ => "$ ",
                };

                eprint!("{}", prompt_symbol)
            }
            PromptLevel::PS2 => {
                let prompt_symbol = match self.sym_tab_stack.get_global_sym_tab().get("PS2") {
                    Some(SymTabEntry::Str { value, .. }) => value,
                    _ => "?> ",
                };

                eprint!("{}", prompt_symbol)
            }
        }
    }
}
