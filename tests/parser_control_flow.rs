use graphol::parser::parse_program;

#[test]
fn rejects_break_outside_while() {
    let err = parse_program("break\n").expect_err("break outside while should fail");
    assert!(err.message.contains("inside loop body"));
}

#[test]
fn rejects_continue_outside_while() {
    let err = parse_program("continue\n").expect_err("continue outside while should fail");
    assert!(err.message.contains("inside loop body"));
}

#[test]
fn rejects_control_flow_as_non_standalone_expression() {
    let source = r#"
i 0
while (< i 10) {
  echo break
}
"#;
    let err = parse_program(source).expect_err("break as argument should fail");
    assert!(err.message.contains("standalone expression"));
}

#[test]
fn accepts_break_and_continue_as_standalone_inside_while() {
    let source = r#"
i 0
while (< i 10) {
  if (< i 5) {
    continue
  }
  break
}
"#;
    parse_program(source).expect("control flow tokens should parse inside loop body");
}
