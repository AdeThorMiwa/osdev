use crate::sym_tab::{SymTab, SymTabEntry};
use home::home_dir;

pub struct Word {
    data: String,
}

pub struct WordList {
    words: Vec<Word>,
}

pub struct WordExpand;

impl ToString for Word {
    fn to_string(&self) -> String {
        self.data.to_string()
    }
}

impl ToString for WordList {
    fn to_string(&self) -> String {
        let mut str = String::new();

        for word in &self.words {
            str.push_str(&word.to_string())
        }

        str
    }
}

impl WordExpand {
    pub fn tilde_expansion(s: &str) -> Option<String> {
        let tilde = "~";
        if !s.contains(tilde) {
            return Some(s.to_string());
        }

        let home_dir = home_dir().unwrap();
        let home_dir_str = home_dir.to_str().unwrap().to_string();
        if s.len() == 1 {
            return Some(home_dir_str);
        }

        // FIXME: silly hack here, look for a better way to go about this
        // e.g rust version of `getpwnam()`
        let user_home_dir = home_dir.parent().unwrap().join(s.replace(tilde, ""));
        if user_home_dir.exists() {
            return Some(user_home_dir.to_str().unwrap().to_string());
        }

        None
    }

    pub fn var_expansion(
        s: &str,
        local_sym_tab: &mut SymTab,
        global_sym_tab: &mut SymTab,
    ) -> Option<String> {
        let mut expand = |str: &str,
                          strip_chars: &[char],
                          default_value_delimiter: Option<&str>,
                          set: bool|
         -> Option<String> {
            let raw = str.replace(strip_chars, "");
            let (symbol, default) = {
                if let Some(delimiter) = default_value_delimiter {
                    let (s, d) = raw.split_once(delimiter).unwrap();
                    (s, Some(d.to_string()))
                } else {
                    (raw.as_str(), None)
                }
            };

            if let Some(SymTabEntry::Str { value, .. }) = local_sym_tab.get(&symbol) {
                return Some(value.to_string());
            }

            if let Some(SymTabEntry::Str { value, .. }) = global_sym_tab.get(&symbol) {
                return Some(value.to_string());
            }

            if set && default.is_some() {
                let entry = SymTabEntry::new_str(symbol, default.clone().unwrap().as_str());
                local_sym_tab.insert(symbol, entry);
            }

            default
        };

        let v = match s {
            // get value length
            str if str.starts_with("${#") => expand(str, &['$', '{', '#', '}'], None, false),
            // get value or use default
            str if str.starts_with("${") && str.contains(":-") => {
                expand(str, &['$', '{', '}'], Some(":-"), false)
            }
            // get value or use default and add to sym_tab
            str if str.starts_with("${") && str.contains(":=") => {
                expand(str, &['$', '{', '}'], Some(":="), true)
            }
            str if str.starts_with("${") => expand(str, &['$', '{', '}'], None, false),
            str if str.starts_with("$") => expand(str, &['$'], None, false),
            _ => None,
        };

        v
    }
}
