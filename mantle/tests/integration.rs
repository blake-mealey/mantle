// #[test_generator::test_resources("mantle/specs/*.yml")]
// fn integration_test(spec: &str) {
#[test]
fn integration_test() {
    let spec = "mantle/specs/first_spec.yml";

    integration_executor::execute_spec(spec);
}
