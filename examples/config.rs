extern crate confargenv;

use std::collections::HashMap;

fn main() {
    // floating=e3.14 bool=etrue cargo run --example config -- --bool=atrue
    // priority: defaults -> conf -> env -> args
    let mut defaults = HashMap::new();
    defaults.insert("string", "dfoo");
    defaults.insert("integer", "d42");
    defaults.insert("floating", "d3.14");
    defaults.insert("bool", "dtrue");

    dbg!(confargenv::fusion(defaults, Some("config")));
}
