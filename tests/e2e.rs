use dsc::config::DiscourseConfig;
use dsc::discourse::{DiscourseClient, GroupDetail};
use dsc::utils::slugify;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use uuid::Uuid;

const DEFAULT_TEST_CONFIG: &str = "testdsc.toml";
const FALLBACK_TEST_CONFIG: &str = "test-dsc.toml";

#[derive(Debug, Deserialize)]
struct TestConfig {
    #[serde(default)]
    discourse: Vec<TestDiscourse>,
}

#[derive(Debug, Deserialize, Clone)]
struct TestDiscourse {
    name: String,
    baseurl: String,
    apikey: String,
    api_username: String,
    changelog_topic_id: Option<u64>,
    ssh_host: Option<String>,
    test_topic_id: Option<u64>,
    test_category_id: Option<u64>,
    test_group_id: Option<u64>,
    ssh_enabled: Option<bool>,
    emoji_path: Option<String>,
    emoji_name: Option<String>,
}

fn load_test_config() -> Option<TestConfig> {
    let path = match std::env::var("TEST_DSC_CONFIG") {
        Ok(path) => path,
        Err(_) => {
            if Path::new(DEFAULT_TEST_CONFIG).exists() {
                DEFAULT_TEST_CONFIG.to_string()
            } else if Path::new(FALLBACK_TEST_CONFIG).exists() {
                FALLBACK_TEST_CONFIG.to_string()
            } else {
                return None;
            }
        }
    };
    let raw = fs::read_to_string(path).ok()?;
    toml::from_str(&raw).ok()
}

fn test_discourse() -> Option<TestDiscourse> {
    load_test_config()?.discourse.into_iter().next()
}

fn test_discourse_pair() -> Option<(TestDiscourse, TestDiscourse)> {
    let mut discourses = load_test_config()?.discourse.into_iter();
    let source = discourses.next()?;
    let target = discourses.next()?;
    Some((source, target))
}

fn to_config(d: &TestDiscourse) -> DiscourseConfig {
    DiscourseConfig {
        name: d.name.clone(),
        baseurl: d.baseurl.clone(),
        apikey: Some(d.apikey.clone()),
        api_username: Some(d.api_username.clone()),
        changelog_topic_id: d.changelog_topic_id,
        ssh_host: d.ssh_host.clone(),
        ..DiscourseConfig::default()
    }
}

fn post_and_verify(d: &TestDiscourse, topic_id: u64, marker: &str) {
    let config = to_config(d);
    let client = DiscourseClient::new(&config).expect("client");
    let body = format!("e2e marker: {}", marker);
    client.create_post(topic_id, &body).expect("post");
    let topic = client.fetch_topic(topic_id, true).expect("fetch topic");
    let found = topic.post_stream.posts.iter().any(|post| {
        post.raw
            .as_ref()
            .map(|raw| raw.contains(marker))
            .unwrap_or(false)
    });
    assert!(found, "marker not found on forum");
}

fn run_dsc(args: &[&str], config_path: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_dsc"))
        .arg("-c")
        .arg(config_path)
        .args(args)
        .output()
        .expect("run dsc")
}

fn write_temp_config(dir: &TempDir, content: &str) -> PathBuf {
    let path = dir.path().join("dsc.toml");
    fs::write(&path, content).expect("write config");
    path
}

#[test]
fn e2e_list() {
    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        "[[discourse]]\nname = \"local\"\nbaseurl = \"https://example.com\"\n",
    );
    let output = run_dsc(&["list", "-f", "json"], &config_path);
    assert!(output.status.success(), "list failed");
}

#[test]
fn e2e_add() {
    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(&dir, "");
    let output = run_dsc(&["add", "newforum"], &config_path);
    assert!(output.status.success(), "add failed");
    let raw = fs::read_to_string(config_path).expect("read config");
    assert!(raw.contains("newforum"));
}

#[test]
fn e2e_import_text() {
    let dir = TempDir::new().expect("tempdir");
    let import_path = dir.path().join("import.txt");
    fs::write(&import_path, "https://example.com\n").expect("write import");
    let config_path = write_temp_config(&dir, "");
    let output = run_dsc(&["import", import_path.to_str().unwrap()], &config_path);
    assert!(output.status.success(), "import failed");
    let raw = fs::read_to_string(config_path).expect("read config");
    assert!(raw.contains("example.com"));
}

#[test]
fn e2e_import_stdin() {
    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(&dir, "");
    let mut child = Command::new(env!("CARGO_BIN_EXE_dsc"))
        .arg("-c")
        .arg(&config_path)
        .arg("import")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("spawn import");
    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin
            .write_all(b"https://example.org\n")
            .expect("write stdin");
    }
    let status = child.wait().expect("wait");
    assert!(status.success(), "import stdin failed");
    let raw = fs::read_to_string(config_path).expect("read config");
    assert!(raw.contains("example.org"));
}

#[test]
fn e2e_update() {
    let Some(test) = test_discourse() else {
        return;
    };
    if test.ssh_enabled != Some(true) {
        return;
    }
    let Some(topic_id) = test.changelog_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    let ssh_host_line = test
        .ssh_host
        .as_ref()
        .map(|host| format!("ssh_host = \"{}\"\n", host))
        .unwrap_or_default();
    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n{}changelog_topic_id = {}\n",
            test.name, test.baseurl, test.apikey, test.api_username, ssh_host_line, topic_id
        ),
    );
    let output = Command::new(env!("CARGO_BIN_EXE_dsc"))
        .arg("-c")
        .arg(&config_path)
        .arg("update")
        .arg(&test.name)
        .arg("-p")
        .env("DSC_TEST_MARKER", &marker)
        .env("DSC_SSH_UPDATE_CMD", "echo update-ok")
        .env("DSC_SSH_CLEANUP_CMD", "echo Total reclaimed space: 0B")
        .output()
        .expect("run update");
    if !output.status.success() {
        panic!(
            "update failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let client = DiscourseClient::new(&to_config(&test)).expect("client");
    let topic = client.fetch_topic(topic_id, true).expect("topic");
    let found = topic.post_stream.posts.iter().any(|post| {
        post.raw
            .as_ref()
            .map(|raw| raw.contains(&marker))
            .unwrap_or(false)
    });
    assert!(found, "marker not found on changelog");
}

#[test]
fn e2e_update_all() {
    let Some(test) = test_discourse() else {
        return;
    };
    if test.ssh_enabled != Some(true) {
        return;
    }
    let Some(topic_id) = test.changelog_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    let ssh_host_line = test
        .ssh_host
        .as_ref()
        .map(|host| format!("ssh_host = \"{}\"\n", host))
        .unwrap_or_default();
    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n{}changelog_topic_id = {}\n",
            test.name, test.baseurl, test.apikey, test.api_username, ssh_host_line, topic_id
        ),
    );
    let output = Command::new(env!("CARGO_BIN_EXE_dsc"))
        .arg("-c")
        .arg(&config_path)
        .arg("update")
        .arg("all")
        .arg("-C")
        .arg("-p")
        .env("DSC_TEST_MARKER", &marker)
        .env("DSC_SSH_UPDATE_CMD", "echo update-ok")
        .env("DSC_SSH_CLEANUP_CMD", "echo Total reclaimed space: 0B")
        .output()
        .expect("run update all");
    if !output.status.success() {
        panic!(
            "update all failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let client = DiscourseClient::new(&to_config(&test)).expect("client");
    let topic = client.fetch_topic(topic_id, true).expect("topic");
    let found = topic.post_stream.posts.iter().any(|post| {
        post.raw
            .as_ref()
            .map(|raw| raw.contains(&marker))
            .unwrap_or(false)
    });
    assert!(found, "marker not found on changelog");
}

#[test]
fn e2e_emoji_add() {
    let Some(test) = test_discourse() else {
        return;
    };
    let Some(topic_id) = test.test_topic_id else {
        return;
    };
    let Some(emoji_path) = test.emoji_path.as_ref() else {
        return;
    };
    let Some(emoji_name) = test.emoji_name.as_ref() else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    post_and_verify(&test, topic_id, &marker);

    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            test.name, test.baseurl, test.apikey, test.api_username
        ),
    );
    let output = run_dsc(
        &["emoji", "add", emoji_path, emoji_name, &test.name],
        &config_path,
    );
    assert!(output.status.success(), "emoji add failed");
}

#[test]
fn e2e_topic_pull() {
    let Some(test) = test_discourse() else {
        return;
    };
    let Some(topic_id) = test.test_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    post_and_verify(&test, topic_id, &marker);

    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            test.name, test.baseurl, test.apikey, test.api_username
        ),
    );
    let output = run_dsc(
        &[
            "topic",
            "pull",
            &topic_id.to_string(),
            dir.path().to_str().unwrap(),
            "--discourse",
            &test.name,
        ],
        &config_path,
    );
    assert!(output.status.success(), "topic pull failed");
}

#[test]
fn e2e_topic_push() {
    let Some(test) = test_discourse() else {
        return;
    };
    let Some(topic_id) = test.test_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    let dir = TempDir::new().expect("tempdir");
    let file_path = dir.path().join("push.md");
    fs::write(&file_path, format!("# E2E Push\n\n{}", marker)).expect("write file");

    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            test.name, test.baseurl, test.apikey, test.api_username
        ),
    );
    let output = run_dsc(
        &[
            "topic",
            "push",
            file_path.to_str().unwrap(),
            &topic_id.to_string(),
            "--discourse",
            &test.name,
        ],
        &config_path,
    );
    assert!(output.status.success(), "topic push failed");
    let config = to_config(&test);
    let client = DiscourseClient::new(&config).expect("client");
    let topic = client.fetch_topic(topic_id, true).expect("topic");
    let found = topic.post_stream.posts.iter().any(|post| {
        post.raw
            .as_ref()
            .map(|raw| raw.contains(&marker))
            .unwrap_or(false)
    });
    assert!(found, "marker not found after push");
}

#[test]
fn e2e_topic_sync() {
    let Some(test) = test_discourse() else {
        return;
    };
    let Some(topic_id) = test.test_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    let dir = TempDir::new().expect("tempdir");
    let file_path = dir.path().join("sync.md");
    fs::write(&file_path, format!("# E2E Sync\n\n{}", marker)).expect("write file");

    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            test.name, test.baseurl, test.apikey, test.api_username
        ),
    );
    let output = run_dsc(
        &[
            "topic",
            "sync",
            &topic_id.to_string(),
            file_path.to_str().unwrap(),
            "--discourse",
            &test.name,
            "--yes",
        ],
        &config_path,
    );
    assert!(output.status.success(), "topic sync failed");
    let config = to_config(&test);
    let client = DiscourseClient::new(&config).expect("client");
    let topic = client.fetch_topic(topic_id, true).expect("topic");
    let found = topic.post_stream.posts.iter().any(|post| {
        post.raw
            .as_ref()
            .map(|raw| raw.contains(&marker))
            .unwrap_or(false)
    });
    assert!(found, "marker not found after sync");
}

#[test]
fn e2e_category_list() {
    let Some(test) = test_discourse() else {
        return;
    };
    let Some(topic_id) = test.test_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    post_and_verify(&test, topic_id, &marker);

    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            test.name, test.baseurl, test.apikey, test.api_username
        ),
    );
    let output = run_dsc(
        &["category", "list", "--discourse", &test.name],
        &config_path,
    );
    assert!(output.status.success(), "category list failed");
}

#[test]
fn e2e_category_copy() {
    let Some((source, target)) = test_discourse_pair() else {
        return;
    };
    let Some(category_id) = source.test_category_id else {
        return;
    };
    let Some(topic_id) = source.test_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    post_and_verify(&source, topic_id, &marker);

    let source_client = DiscourseClient::new(&to_config(&source)).expect("client");
    let source_categories = source_client.fetch_categories().expect("categories");
    let source_category = source_categories
        .iter()
        .find(|cat| cat.id == Some(category_id))
        .expect("source category");

    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n\n[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            source.name,
            source.baseurl,
            source.apikey,
            source.api_username,
            target.name,
            target.baseurl,
            target.apikey,
            target.api_username
        ),
    );
    let output = run_dsc(
        &[
            "category",
            "copy",
            "--source",
            &source.name,
            "--target",
            &target.name,
            &category_id.to_string(),
        ],
        &config_path,
    );
    assert!(output.status.success(), "category copy failed");
    let target_client = DiscourseClient::new(&to_config(&target)).expect("client");
    let target_categories = target_client.fetch_categories().expect("categories");
    let found = target_categories
        .iter()
        .any(|cat| cat.name == source_category.name);
    assert!(found, "copied category not found on target");
}

#[test]
fn e2e_category_pull() {
    let Some(test) = test_discourse() else {
        return;
    };
    let Some(category_id) = test.test_category_id else {
        return;
    };
    let Some(topic_id) = test.test_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    post_and_verify(&test, topic_id, &marker);

    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            test.name, test.baseurl, test.apikey, test.api_username
        ),
    );
    let output = run_dsc(
        &[
            "category",
            "pull",
            &category_id.to_string(),
            dir.path().to_str().unwrap(),
            "--discourse",
            &test.name,
        ],
        &config_path,
    );
    assert!(output.status.success(), "category pull failed");
}

#[test]
fn e2e_category_push() {
    let Some(test) = test_discourse() else {
        return;
    };
    let Some(category_id) = test.test_category_id else {
        return;
    };
    let Some(topic_id) = test.test_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    post_and_verify(&test, topic_id, &marker);

    let dir = TempDir::new().expect("tempdir");
    let file_path = dir.path().join("category-push.md");
    let title = format!("E2E Category Push {}", &marker);
    fs::write(&file_path, format!("# {}\n\n{}", title, marker)).expect("write file");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            test.name, test.baseurl, test.apikey, test.api_username
        ),
    );
    let output = run_dsc(
        &[
            "category",
            "push",
            dir.path().to_str().unwrap(),
            &category_id.to_string(),
            "--discourse",
            &test.name,
        ],
        &config_path,
    );
    assert!(output.status.success(), "category push failed");
    let config = to_config(&test);
    let client = DiscourseClient::new(&config).expect("client");
    let category = client.fetch_category(category_id).expect("category");
    let found = category
        .topic_list
        .topics
        .iter()
        .any(|topic| topic.title.contains(&marker));
    assert!(found, "new category topic not found");
}

#[test]
fn e2e_group_list() {
    let Some(test) = test_discourse() else {
        return;
    };
    let Some(topic_id) = test.test_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    post_and_verify(&test, topic_id, &marker);

    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            test.name, test.baseurl, test.apikey, test.api_username
        ),
    );
    let output = run_dsc(&["group", "list", "--discourse", &test.name], &config_path);
    assert!(output.status.success(), "group list failed");
}

#[test]
fn e2e_group_copy() {
    let Some((source, target)) = test_discourse_pair() else {
        return;
    };
    let Some(group_id) = source.test_group_id else {
        return;
    };
    let Some(topic_id) = source.test_topic_id else {
        return;
    };
    let marker = Uuid::new_v4().to_string();
    post_and_verify(&source, topic_id, &marker);

    let source_client = DiscourseClient::new(&to_config(&source)).expect("client");
    let source_groups = source_client.fetch_groups().expect("groups");
    let source_group = source_groups
        .iter()
        .find(|group| group.id == group_id)
        .expect("source group");

    let dir = TempDir::new().expect("tempdir");
    let config_path = write_temp_config(
        &dir,
        &format!(
            "[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n\n[[discourse]]\nname = \"{}\"\nbaseurl = \"{}\"\napikey = \"{}\"\napi_username = \"{}\"\n",
            source.name,
            source.baseurl,
            source.apikey,
            source.api_username,
            target.name,
            target.baseurl,
            target.apikey,
            target.api_username
        ),
    );
    let output = run_dsc(
        &[
            "group",
            "copy",
            "--source",
            &source.name,
            "--target",
            &target.name,
            &group_id.to_string(),
        ],
        &config_path,
    );
    assert!(output.status.success(), "group copy failed");
    let target_client = DiscourseClient::new(&to_config(&target)).expect("client");
    let target_groups = target_client.fetch_groups().expect("groups");
    let found = target_groups
        .iter()
        .find(|group| group.name == format!("{}-copy", slugify(&source_group.name)));
    let Some(found) = found else {
        panic!("copied group not found on target");
    };
    let source_detail = source_client
        .fetch_group_detail(source_group.id, Some(&source_group.name))
        .expect("source detail");
    let target_detail = target_client
        .fetch_group_detail(found.id, Some(&found.name))
        .expect("target detail");
    let source_fields = group_settings(&source_detail);
    let target_fields = group_settings(&target_detail);
    assert_eq!(
        target_fields.get("name"),
        Some(&format!("{}-copy", slugify(&source_detail.name))),
        "copy name mismatch"
    );
    if let Some(full_name) = source_detail.full_name.as_deref() {
        assert_eq!(
            target_fields.get("full_name"),
            Some(&format!("Copy of {}", full_name)),
            "copy full name mismatch"
        );
    }
    let mut expected_fields = source_fields.clone();
    expected_fields.insert(
        "name".to_string(),
        format!("{}-copy", slugify(&source_detail.name)),
    );
    if let Some(full_name) = source_detail.full_name.as_deref() {
        expected_fields.insert("full_name".to_string(), format!("Copy of {}", full_name));
    }
    assert_eq!(expected_fields, target_fields, "group settings differ");
}

fn group_settings(detail: &GroupDetail) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    insert_opt(&mut map, "name", Some(&detail.name));
    if let Some(full_name) = detail.full_name.as_deref() {
        insert_opt(&mut map, "full_name", Some(full_name));
    }
    insert_opt(&mut map, "title", detail.title.as_deref());
    insert_opt(
        &mut map,
        "grant_trust_level",
        detail.grant_trust_level.map(|v| v.to_string()).as_deref(),
    );
    insert_opt(
        &mut map,
        "visibility_level",
        detail.visibility_level.map(|v| v.to_string()).as_deref(),
    );
    insert_opt(
        &mut map,
        "mentionable_level",
        detail.mentionable_level.map(|v| v.to_string()).as_deref(),
    );
    insert_opt(
        &mut map,
        "messageable_level",
        detail.messageable_level.map(|v| v.to_string()).as_deref(),
    );
    insert_opt(
        &mut map,
        "default_notification_level",
        detail
            .default_notification_level
            .map(|v| v.to_string())
            .as_deref(),
    );
    insert_opt(
        &mut map,
        "members_visibility_level",
        detail
            .members_visibility_level
            .map(|v| v.to_string())
            .as_deref(),
    );
    insert_opt(
        &mut map,
        "primary_group",
        detail.primary_group.map(|v| v.to_string()).as_deref(),
    );
    insert_opt(
        &mut map,
        "public_admission",
        detail.public_admission.map(|v| v.to_string()).as_deref(),
    );
    insert_opt(
        &mut map,
        "public_exit",
        detail.public_exit.map(|v| v.to_string()).as_deref(),
    );
    insert_opt(
        &mut map,
        "allow_membership_requests",
        detail
            .allow_membership_requests
            .map(|v| v.to_string())
            .as_deref(),
    );
    insert_opt(
        &mut map,
        "automatic_membership_email_domains",
        detail.automatic_membership_email_domains.as_deref(),
    );
    insert_opt(
        &mut map,
        "automatic_membership_retroactive",
        detail
            .automatic_membership_retroactive
            .map(|v| v.to_string())
            .as_deref(),
    );
    insert_opt(
        &mut map,
        "membership_request_template",
        detail.membership_request_template.as_deref(),
    );
    insert_opt(&mut map, "flair_icon", detail.flair_icon.as_deref());
    insert_opt(
        &mut map,
        "flair_upload_id",
        detail.flair_upload_id.map(|v| v.to_string()).as_deref(),
    );
    insert_opt(&mut map, "flair_color", detail.flair_color.as_deref());
    insert_opt(
        &mut map,
        "flair_background_color",
        detail.flair_background_color.as_deref(),
    );
    insert_opt(&mut map, "bio_raw", detail.bio_raw.as_deref());
    map
}

fn insert_opt(map: &mut BTreeMap<String, String>, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        map.insert(key.to_string(), value.to_string());
    }
}
