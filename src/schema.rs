mod lib;

use lib::config::Config;
use schemars::gen::SchemaSettings;

fn main() {
    let settings = SchemaSettings::draft07().with(|s| {
        s.inline_subschemas = true;
    });
    let gen = settings.into_generator();
    let schema = gen.into_root_schema_for::<Config>();

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
