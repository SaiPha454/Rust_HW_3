use assert_cmd::Command;
use lab3::tem_table::convert_temparature; //import the function from the module from lib.rs

#[test]
fn test_temperature_table() {
    let mut cmd = Command::cargo_bin("tem_table").unwrap();
    cmd.args(vec!["100", "105"]);
    cmd.assert().success();

}

#[test]
fn test_temperature_table_with_stdout() {

    let mut cmd = Command::cargo_bin("tem_table").unwrap();
    cmd.args(vec!["0", "300"]);
    cmd.assert().success().stdout(convert_temparature(0, 300)+"\n");

}

#[test]
fn test_temperature_table_in_reverse_with_stdout() {

    let mut cmd = Command::cargo_bin("tem_table").unwrap();
    cmd.args(vec!["300", "0"]);
    cmd.assert().success().stdout(convert_temparature(300, 0)+"\n");

}