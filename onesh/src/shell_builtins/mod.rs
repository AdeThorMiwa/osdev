use crate::builtin::BuiltIn;

use self::currenttime::current_time;

pub mod currenttime;

pub fn builtins() -> Vec<BuiltIn> {
    let currenttime = BuiltIn {
        name: "currenttime".to_string(),
        func: current_time,
    };

    vec![currenttime]
}
