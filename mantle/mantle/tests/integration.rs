#[test_generator::test_resources("specs/*.yml")]
fn integration_test(spec: &str) {
    // #[test]
    // fn integration_test() {
    //     let spec = "specs/thumbnails.yml";

    integration_executor::execute_spec(spec);
}
