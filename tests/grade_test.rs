use assert_cmd::Command;


#[test]
fn test_grade_score () {
    let mut cmd = Command::cargo_bin("grade_score").unwrap();
    cmd.arg("80").assert().success();
}

#[test]
fn test_grade_score_with_stdout () {
    let mut cmd = Command::cargo_bin("grade_score").unwrap();
    cmd.arg("80");
    cmd.assert().success().stdout("Grade : B\n");
}

#[test]
fn test_grade_score_with_validator_over100 () {
    let mut cmd = Command::cargo_bin("grade_score").unwrap();
    cmd.arg("101");
    cmd.assert().failure();
}

#[test]
fn test_grade_score_with_validator_lower_than_0 () {
    let mut cmd = Command::cargo_bin("grade_score").unwrap();
    cmd.arg("-10");
    cmd.assert().failure();
}