use crate::cli::CliContext;
use home;
use std::{env, path::PathBuf};

pub fn cd(_argc: usize, argv: Vec<String>, _ctx: &mut CliContext) {
    let home_dir = home::home_dir().unwrap();

    let given_dir = match argv.iter().nth(1) {
        None => "".to_string(),
        Some(v) => v.trim().replace("~", home_dir.to_str().unwrap()),
    };

    let current_dir = env::current_dir().unwrap();

    let new_dir = match given_dir {
        dir if dir.is_empty() => home::home_dir().unwrap(),
        dir if dir == "." => return,
        dir if dir == ".." || dir == "../" => {
            if let Some(parent) = current_dir.parent() {
                PathBuf::from(parent)
            } else {
                return;
            }
        }
        dir if !dir.starts_with('/') => current_dir.join(dir),
        _ => PathBuf::from(given_dir.trim()),
    };

    let _ = env::set_current_dir(new_dir);
}
