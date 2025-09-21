use assert_cmd::Command;

#[test]
fn help_displays() {
    Command::cargo_bin("milkshake")
        .expect("binary exists")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("milkshake"));
}
