#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/protocol_read.rs");
}
