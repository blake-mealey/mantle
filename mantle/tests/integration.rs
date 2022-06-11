// #[test_generator::test_resources("mantle/specs/*.yml")]
// fn integration_test(spec: &str) {
#[test]
fn integration_test() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("off")).init();

    let spec = "mantle/specs/first_spec.yml";

    integration_executor::execute_spec(spec);
}
