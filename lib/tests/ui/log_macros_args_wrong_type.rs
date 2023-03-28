//! Test too many arguments for log_macros

use lib_common::log_macros;

log_macros!(not_a_StrLit);

fn main() {}
