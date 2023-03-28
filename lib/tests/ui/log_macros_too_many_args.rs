//! Test too many arguments for log_macros

use lib_common::log_macros;

log_macros!("one", "two", "three");

fn main() {}
