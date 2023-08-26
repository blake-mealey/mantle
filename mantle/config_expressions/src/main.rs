use std::{collections::HashMap, error::Error, fs::File};

use cel_interpreter::{CelType, Context, ExecutionError, FunctionCtx};
use config_expressions::{evaluate_expressions, yaml_value_to_cel_value};
use serde::Deserialize;
use serde_yaml::Value;

#[derive(Deserialize)]
struct MetaConfig {
    environments: Vec<EnvironmentConfig>,
}

#[derive(Deserialize)]
struct EnvironmentConfig {
    label: String,
    variables: HashMap<String, Value>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = File::open("demo.yml")?;
    // let mut config: Value = serde_yaml::from_reader(config_file)?;

    let mut meta_config: Option<MetaConfig> = None;
    let mut config: Option<Value> = None;
    for document in serde_yaml::Deserializer::from_reader(config_file) {
        if meta_config.is_none() {
            meta_config = Some(MetaConfig::deserialize(document)?);
        } else if config.is_none() {
            config = Some(Value::deserialize(document)?);
        } else {
            break;
        }
    }

    let env = "dev";

    if let (Some(meta_config), Some(config)) = (&meta_config, &mut config) {
        let mut context = Context::default();
        context.add_function("upper", upper);
        context.add_function("lower", lower);

        let env_config = meta_config
            .environments
            .iter()
            .find(|e| e.label == env)
            .unwrap();

        context.add_variable("environmentLabel", env.to_owned());

        for (name, value) in &env_config.variables {
            context.add_variable(name, yaml_value_to_cel_value(value.clone()));
        }

        let config_map = config.as_mapping_mut().unwrap();
        for value in config_map.values_mut() {
            evaluate_expressions(value, &context).unwrap();
        }

        println!("{}", serde_yaml::to_string(&config).unwrap());
    }

    Ok(())
}

pub fn upper(ftx: FunctionCtx) -> Result<CelType, ExecutionError> {
    Ok(match ftx.target()? {
        CelType::String(v) => CelType::String(v.clone().to_uppercase().into()),
        _ => ftx.error("only strings are supported")?,
    })
}

pub fn lower(ftx: FunctionCtx) -> Result<CelType, ExecutionError> {
    Ok(match ftx.target()? {
        CelType::String(v) => CelType::String(v.clone().to_lowercase().into()),
        _ => ftx.error("only strings are supported")?,
    })
}
