mod common;
use common::*;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn theme_list() {
    let Some(test) = test_discourse() else {
        return;
    };
    vprintln("e2e_theme_list: listing themes");
    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            test.name, test.baseurl, test.apikey, test.api_username
        ),
    );
    let output = run_dsc(&["theme", "list", &test.name], &config_path);
    assert!(output.status.success(), "theme list failed");
}

#[test]
fn theme_install_remove() {
    let Some(test) = test_discourse() else {
        return;
    };
    if test.ssh_enabled != Some(true) {
        return;
    }
    let Some(url) = test.test_theme_url.as_ref() else {
        return;
    };
    let Some(name) = test.test_theme_name.as_ref() else {
        return;
    };
    vprintln("e2e_theme_install_remove: install/remove theme");
    let ssh_host_line = test
        .ssh_host
        .as_ref()
        .map(|host| format!("ssh_host = \"{}\"\n", host))
        .unwrap_or_default();
    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n{}",
            test.name, test.baseurl, test.apikey, test.api_username, ssh_host_line
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_dsc"))
        .arg("-c")
        .arg(&config_path)
        .arg("theme")
        .arg("install")
        .arg(&test.name)
        .arg(url)
        .env("DSC_SSH_THEME_INSTALL_CMD", "echo theme install {url}")
        .output()
        .expect("run theme install");
    assert!(output.status.success(), "theme install failed");

    let output = Command::new(env!("CARGO_BIN_EXE_dsc"))
        .arg("-c")
        .arg(&config_path)
        .arg("theme")
        .arg("remove")
        .arg(&test.name)
        .arg(name)
        .env("DSC_SSH_THEME_REMOVE_CMD", "echo theme remove {name}")
        .output()
        .expect("run theme remove");
    assert!(output.status.success(), "theme remove failed");
}
