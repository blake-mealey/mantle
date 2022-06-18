#[macro_export]
macro_rules! all_outputs {
    ($expr:expr, $enum:path) => {{
        $expr
            .iter()
            .filter_map(|value| {
                if let $enum(outputs) = value {
                    Some(outputs)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }};
}

#[macro_export]
macro_rules! single_output {
    ($expr:expr, $enum:path) => {{
        *all_outputs!($expr, $enum)
            .first()
            .expect("Missing expected output")
    }};
}

#[macro_export]
macro_rules! optional_output {
    ($expr:expr, $enum:path) => {{
        all_outputs!($expr, $enum).first().map(|output| *output)
    }};
}
