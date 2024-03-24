use crate::builtin::BuiltIn;
pub mod cd;
pub mod currenttime;
pub mod dump;
pub mod pwd;

use self::cd::cd;
use self::currenttime::current_time;
use self::dump::dump;
use self::pwd::pwd;

pub fn builtins() -> Vec<BuiltIn> {
    let currenttime = BuiltIn {
        name: "currenttime".to_string(),
        func: current_time,
    };

    let dump_b = BuiltIn {
        name: "dump".to_string(),
        func: dump,
    };

    let cd_b = BuiltIn {
        name: "cd".to_string(),
        func: cd,
    };

    let pwd_b = BuiltIn {
        name: "pwd".to_string(),
        func: pwd,
    };

    vec![currenttime, dump_b, cd_b, pwd_b]
}
