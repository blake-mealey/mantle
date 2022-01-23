mod lib;

use lib::config::Config;
use schemars::schema_for;

fn main() {
    let schema = schema_for!(Config);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
