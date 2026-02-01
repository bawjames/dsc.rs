use common::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn completions_generate() {
    vprintln("e2e_completions: generate completions");
    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(&dir, "");
    let out_dir = dir.path().join("completions");
    let out_dir_str = out_dir.to_str().expect("out dir");

    let output = run_dsc(&["completions", "bash", "--dir", out_dir_str], &config_path);
    assert!(output.status.success(), "bash completions failed");
    assert!(out_dir.join("dsc.bash").exists(), "missing dsc.bash");

    let output = run_dsc(&["completions", "zsh", "--dir", out_dir_str], &config_path);
    assert!(output.status.success(), "zsh completions failed");
    assert!(out_dir.join("_dsc").exists(), "missing _dsc");

    let output = run_dsc(&["completions", "fish", "--dir", out_dir_str], &config_path);
    assert!(output.status.success(), "fish completions failed");
    assert!(out_dir.join("dsc.fish").exists(), "missing dsc.fish");

    let entries: Vec<_> = fs::read_dir(&out_dir)
        .expect("read completions dir")
        .filter_map(|entry| entry.ok())
        .collect();
    assert!(entries.len() >= 3, "unexpected completions count");
}
