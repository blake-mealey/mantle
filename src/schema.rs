use rbx_mantle::config::Config;
use schemars::{
    gen::SchemaSettings,
    schema::SchemaObject,
    visit::{visit_schema_object, Visitor},
};

#[derive(Debug, Clone)]
pub struct MarkdownVisitor;

impl Visitor for MarkdownVisitor {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        if let Some(metadata) = schema.metadata.as_mut() {
            if let Some(desc) = metadata.description.as_mut() {
                let mut default: Option<String> = None;
                let mut skip_properties: bool = false;
                *desc = desc
                    .lines()
                    .filter(|line| {
                        if line.starts_with("default(") {
                            default = Some(line[8..line.len() - 1].to_owned());
                            false
                        } else if line.starts_with("skip_properties()") {
                            skip_properties = true;
                            false
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                metadata.default = default.map(serde_json::Value::String);
                if skip_properties {
                    schema.extensions.insert(
                        "x-skip-properties".to_owned(),
                        serde_json::Value::Bool(true),
                    );
                }
            }
        }

        visit_schema_object(self, schema);
    }
}

fn main() {
    let schema = SchemaSettings::draft07()
        .with_visitor(MarkdownVisitor)
        .with(|s| {
            s.inline_subschemas = true;
            s.option_add_null_type = false;
        })
        .into_generator()
        .into_root_schema_for::<Config>();

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
