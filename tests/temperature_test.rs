use assert_cmd::Command;
use lab3::tem_table::convert_temparature; //import the function from the module from lib.rs

#[test]
fn test_temperature_table() {
    let mut cmd = Command::cargo_bin("temperature_table").unwrap();
    cmd.args(vec!["0", "300", "20"]);
    cmd.assert().success();

}

#[test]
fn test_temperature_table_with_stdout() {

    let mut cmd = Command::cargo_bin("temperature_table").unwrap();
    cmd.args(vec!["0", "280", "40"]);
    cmd.assert().success().stdout(convert_temparature(0, 300, 40)+"\n");

}

#[test]
fn test_temperature_table_in_reverse_with_stdout() {

    let mut cmd = Command::cargo_bin("temperature_table").unwrap();
    cmd.args(vec!["300", "0" , "20"]);
    cmd.assert().success().stdout(convert_temparature(300, 0, 20)+"\n");

}