use assert_cmd::Command;


#[test]
fn print_arrow_test_with_arg_0() {
    let mut cmd = Command::cargo_bin("arrow").unwrap();
    cmd.arg("0");

    let expected = "";
    cmd.assert().success().stdout(expected);
}

#[test]
fn print_arrow_test_with_arg_1() {
    let mut cmd = Command::cargo_bin("arrow").unwrap();
    cmd.arg("1");

    let expected = "*\n";
    cmd.assert().success().stdout(expected);
}

#[test]
fn print_arrow_test_with_arg_2() {
    let mut cmd = Command::cargo_bin("arrow").unwrap();
    cmd.arg("2");

    let expected = "*\n**\n*\n";
    cmd.assert().success().stdout(expected);
}

#[test]
fn print_arrow_test_with_arg_3() {
    let mut cmd = Command::cargo_bin("arrow").unwrap();
    cmd.arg("3");

    let expected = "*\n**\n***\n**\n*\n";
    cmd.assert().success().stdout(expected);
}


