use std::process::Command;

fn invoke(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_validator"))
        .args(arguments)
        .output()
        .expect("start validator")
}

#[test]
fn top_level_help_lists_every_supported_command() {
    let result = invoke(&["--help"]);
    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    let help = String::from_utf8(result.stdout).expect("UTF-8 help");
    for command in ["evaluate", "compare"] {
        assert!(help.contains(command), "missing {command}");
    }
    assert!(help.contains("not that the classifier met a quality threshold"));
}

#[test]
fn each_command_has_discoverable_help_without_opening_inputs() {
    for command in ["evaluate", "compare"] {
        for flag in ["--help", "-h"] {
            let result = invoke(&[command, flag]);
            assert!(result.status.success(), "{command} {flag}");
            assert!(result.stderr.is_empty());
            let help = String::from_utf8(result.stdout).expect("UTF-8 help");
            assert!(help.starts_with(&format!("validator {command} ")));
            if command == "evaluate" {
                assert!(help.contains("--out is required"));
                assert!(!help.contains("[--out"));
            }
            if command == "compare" {
                assert!(help.contains("REPORT [REPORT ...]"));
                assert!(!help.contains("--baseline"));
            }
        }
    }
}

#[test]
fn unknown_command_help_still_returns_the_machine_error_contract() {
    let result = invoke(&["unknown", "--help"]);
    assert_eq!(result.status.code(), Some(2));
    let error: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(error["kind"], "error");
    assert_eq!(error["code"], "E_SCHEMA");
}
