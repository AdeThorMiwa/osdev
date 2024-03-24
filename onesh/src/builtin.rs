pub type BuiltInFuncType = fn(argc: usize, argv: Vec<String>);

#[derive(Debug, Clone)]
pub struct BuiltIn {
    pub name: String,
    pub func: BuiltInFuncType,
}

pub struct BuiltInList<'a> {
    builtins: Vec<&'a BuiltIn>,
    index: usize,
}

impl<'a> BuiltInList<'a> {
    pub fn from(builtins: Vec<&'a BuiltIn>) -> Self {
        Self { builtins, index: 0 }
    }

    pub fn len(&self) -> usize {
        self.builtins.len()
    }
}

impl<'a> Iterator for BuiltInList<'a> {
    type Item = &'a BuiltIn;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.builtins.len() {
            let next = self.builtins[self.index];
            self.index += 1;
            Some(next)
        } else {
            None
        }
    }
}
