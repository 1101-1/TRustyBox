#[test]
pub fn short_url() {
    use trustybox_tests::tools::short_url;
    assert_eq!(short_url::generate_short_path_url().len(), 8);
}