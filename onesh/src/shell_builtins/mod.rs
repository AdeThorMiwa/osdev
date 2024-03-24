use crate::builtin::BuiltIn;
pub mod currenttime;
pub mod dump;

use self::currenttime::current_time;
use self::dump::dump;

pub fn builtins() -> Vec<BuiltIn> {
    let currenttime = BuiltIn {
        name: "currenttime".to_string(),
        func: current_time,
    };

    let dump_b = BuiltIn {
        name: "dump".to_string(),
        func: dump,
    };

    vec![currenttime, dump_b]
}
