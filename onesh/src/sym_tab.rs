use crate::builtin::BuiltInFuncType;
use std::collections::HashMap;

#[derive(Debug)]
pub enum SymTabEntry {
    Str {
        symbol: String,
        flags: usize,
        value: String,
    },
    Func {
        symbol: String,
        func_body: BuiltInFuncType,
    },
}

impl SymTabEntry {
    pub fn new_str(symbol: &str, value: &str) -> Self {
        Self::Str {
            symbol: symbol.to_string(),
            flags: 0,
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SymTab {
    #[allow(dead_code)]
    level: usize,
    entries: HashMap<String, SymTabEntry>,
}

#[derive(Debug)]
pub struct SymTabStack {
    stack: Vec<SymTab>,
}

impl SymTab {
    pub fn new(level: usize) -> Self {
        Self {
            level,
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, symbol: &str, value: SymTabEntry) -> Option<SymTabEntry> {
        self.entries.insert(symbol.to_string(), value)
    }

    pub fn get(&self, symbol: &str) -> Option<&SymTabEntry> {
        self.entries.get(symbol)
    }

    pub fn delete(&mut self, symbol: &str) -> Option<SymTabEntry> {
        self.entries.remove(symbol)
    }
}

impl SymTabStack {
    pub fn new() -> Self {
        let mut internal_stack = Vec::with_capacity(Self::max_sym_tab());
        let global_sym_tab = SymTab::new(0);
        internal_stack.push(global_sym_tab);
        Self {
            stack: internal_stack,
        }
    }

    pub fn push(&mut self, entry: SymTab) {
        self.stack.push(entry);
    }

    pub fn pop(&mut self) -> Option<SymTab> {
        // prevent popping global stack
        if self.stack.len() == 1 {
            return None;
        }

        self.stack.pop()
    }

    pub fn get_global_sym_tab_mut(&mut self) -> &mut SymTab {
        let glob = &mut self.stack[0];
        glob
    }

    pub fn get_global_sym_tab(&self) -> &SymTab {
        let glob = &self.stack[0];
        glob
    }

    pub fn get_local_sym_tab(&self) -> Option<&SymTab> {
        let index = self.stack.len() - 1;
        let stack = &self.stack;
        stack.into_iter().nth(index)
    }

    pub fn max_sym_tab() -> usize {
        256
    }
}
