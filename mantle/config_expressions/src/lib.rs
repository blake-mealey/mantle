use std::collections::HashMap;

use cel_interpreter::{objects::CelKey, CelType, Context, Program};
use serde_yaml::Value;
use thiserror::Error;

const EXPR_START: &str = "${";
const EXPR_END: &str = "}";

#[derive(Error, Debug)]
pub enum EvaluateError {
    #[error("error parsing expression: {0}")]
    ParseError(#[from] cel_interpreter::ParseError),
    #[error("error evaluating expression: {0}")]
    EvaluateError(#[from] cel_interpreter::ExecutionError),
}

pub fn evaluate_expressions(value: &mut Value, context: &Context) -> Result<(), EvaluateError> {
    match value {
        Value::String(value_str) => {
            let expressions = find_all_expressions(value_str);

            // When the whole value is an expression, convert the result back to a YAML value and return it
            // directly (e.g. a cel map will be represented as a YAML map). Otherwise, replace each expression
            // with its result converted to a string.
            if expressions.len() == 1
                && expressions[0].start == 0
                && expressions[0].end == value_str.len()
            {
                let expr = expressions[0].value;
                let program = Program::compile(expr)?;
                let result = program.execute(&context)?;
                *value = cel_value_to_yaml_value(result);
            } else {
                let mut new_value = String::new();
                let mut pos = 0;
                for expression in expressions {
                    let expr = expression.value;
                    let program = Program::compile(expr)?;
                    let result = program.execute(&context)?;

                    new_value.push_str(&value_str[pos..expression.start]);
                    new_value.push_str(&cell_value_to_str(result));
                    pos = expression.end + EXPR_END.len() - 1;
                }
                new_value.push_str(&value_str[pos..]);
                *value = Value::String(new_value);
            }
        }
        Value::Mapping(map) => {
            for value in map.values_mut() {
                evaluate_expressions(value, &context).unwrap();
            }
        }
        Value::Sequence(sequence) => {
            for value in sequence {
                evaluate_expressions(value, &context).unwrap();
            }
        }
        Value::Tagged(tagged) => {
            evaluate_expressions(&mut tagged.value, &context).unwrap();
        }
        _ => {}
    }

    Ok(())
}

struct Expression<'a> {
    start: usize,
    end: usize,
    value: &'a str,
}

fn find_all_expressions<'a>(value: &'a str) -> Vec<Expression<'a>> {
    let mut expressions = vec![];

    let mut start = 0;

    while let Some(start_index) = value[start..].find(EXPR_START) {
        start += start_index;
        let end_index = value[start..].find(EXPR_END).map(|end| start + end + 1);

        if let Some(end_index) = end_index {
            let end = end_index;

            expressions.push(Expression {
                start,
                end,
                value: &value[start + EXPR_START.len()..end - 1],
            });

            start = end;
        } else {
            break;
        }
    }

    expressions
}

fn cell_value_to_str(value: CelType) -> String {
    match value {
        CelType::List(_) => serde_yaml::to_string(&cel_value_to_yaml_value(value)).unwrap(),
        CelType::Map(_) => serde_yaml::to_string(&cel_value_to_yaml_value(value)).unwrap(),
        CelType::Function(_, _) => unimplemented!(),
        CelType::Int(x) => x.to_string(),
        CelType::UInt(x) => x.to_string(),
        CelType::Float(x) => x.to_string(),
        CelType::String(x) => x.to_string(),
        CelType::Bytes(x) => String::from_utf8_lossy(&x).to_string(),
        CelType::Bool(x) => x.to_string(),
        CelType::Duration(x) => x.to_string(),
        CelType::Timestamp(x) => x.to_string(),
        CelType::Null => "null".to_owned(),
    }
}

fn cel_value_to_yaml_value(value: CelType) -> Value {
    match value {
        CelType::List(x) => {
            Value::Sequence(x.iter().cloned().map(cel_value_to_yaml_value).collect())
        }
        CelType::Map(x) => Value::Mapping(
            x.map
                .iter()
                .map(|(k, v)| {
                    (
                        cel_key_to_yaml_key(k.clone()),
                        cel_value_to_yaml_value(v.clone()),
                    )
                })
                .collect(),
        ),
        CelType::Function(_, _) => unimplemented!(),
        CelType::Int(x) => Value::Number(x.into()),
        CelType::UInt(x) => Value::Number(x.into()),
        CelType::Float(x) => Value::Number(x.into()),
        CelType::String(x) => Value::String(x.to_string()),
        CelType::Bytes(x) => Value::String(String::from_utf8_lossy(&x).to_string()),
        CelType::Bool(x) => Value::Bool(x),
        CelType::Duration(x) => Value::String(x.to_string()),
        CelType::Timestamp(x) => Value::String(x.to_string()),
        CelType::Null => Value::Null,
    }
}

fn cel_key_to_yaml_key(value: CelKey) -> Value {
    match value {
        CelKey::Int(x) => Value::Number(x.into()),
        CelKey::Uint(x) => Value::Number(x.into()),
        CelKey::Bool(x) => Value::Bool(x),
        CelKey::String(x) => Value::String(x.to_string()),
    }
}

pub fn yaml_value_to_cel_value(value: Value) -> CelType {
    match value {
        Value::Null => CelType::Null,
        Value::Bool(x) => CelType::Bool(x),
        Value::Number(x) => {
            if x.is_i64() {
                CelType::Int(x.as_i64().unwrap().try_into().unwrap())
            } else if x.is_u64() {
                CelType::UInt(x.as_u64().unwrap().try_into().unwrap())
            } else if x.is_f64() {
                CelType::Float(x.as_f64().unwrap())
            } else {
                unreachable!()
            }
        }
        Value::String(x) => CelType::String(x.into()),
        Value::Sequence(x) => CelType::List(
            x.into_iter()
                .map(yaml_value_to_cel_value)
                .collect::<Vec<_>>()
                .into(),
        ),
        Value::Mapping(x) => CelType::Map(
            x.into_iter()
                .map(|(k, v)| (yaml_value_to_cel_key(k), yaml_value_to_cel_value(v)))
                .collect::<HashMap<_, _>>()
                .into(),
        ),
        Value::Tagged(x) => yaml_value_to_cel_value(x.value),
    }
}

fn yaml_value_to_cel_key(value: Value) -> CelKey {
    match value {
        Value::Null => CelKey::String("null".to_owned().into()),
        Value::Bool(x) => CelKey::Bool(x),
        Value::Number(x) => {
            if x.is_i64() {
                CelKey::Int(x.as_i64().unwrap().try_into().unwrap())
            } else if x.is_u64() {
                CelKey::Uint(x.as_u64().unwrap().try_into().unwrap())
            } else if x.is_f64() {
                CelKey::String(x.as_f64().unwrap().to_string().into())
            } else {
                unreachable!()
            }
        }
        Value::String(x) => CelKey::String(x.into()),
        Value::Sequence(_) => CelKey::String(serde_yaml::to_string(&value).unwrap().into()),
        Value::Mapping(_) => CelKey::String(serde_yaml::to_string(&value).unwrap().into()),
        Value::Tagged(x) => yaml_value_to_cel_key(x.value),
    }
}
