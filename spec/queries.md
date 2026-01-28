## Questions

1. For `dsc update`, how should SSH be configured? Should we add fields like `ssh_host`/`ssh_user` (or a full SSH config name) in `dsc.toml`, or should update skip SSH entirely and only post the changelog?

A: SSH hosts should be configured via the user's SSH config file. `dsc.toml` will not store any SSH credentials. That way we keen dsc.rs optional, there is always a working SSH config setup.

2. `dsc import` currently requires a file path. Is that acceptable, or should it also support stdin?

A: Yes, supporting stdin would be a good idea for flexibility.

3. `dsc add <name>,<name>,...` currently creates placeholder entries with empty `baseurl`/`apikey`. Should it prompt for `baseurl` (or accept flags) instead?

A: dsc add should have a -i flag for interactive mode, prompting for baseurl/apikey. Otherwise creating placeholders is fine.

4. Topic/category commands do not specify a Discourse name in the spec. Is adding `--discourse <name>` (or defaulting when only one config exists) OK?

A: there should be a `--discourse <name>` flag for all commands that need to target a specific Discourse install. If only one install exists in dsc.toml, it can default to that. It could be shortened to `-d <name>` for convenience.

5. Discourse API calls need `api_username` in addition to `apikey`. Can we add `api_username` to `dsc.toml` as shown in `dsc.example.toml`?

A: Yes, adding `api_username` to `dsc.toml` is necessary for Discourse API calls and should be included in the example file.

6. `dsc update` posts a fixed checklist to `changelog_topic_id`. Do you want extra fields like version/reclaimed-space flags to fill in the placeholders?

A: Yes we need to collect that data as the update runs and it should automatically feed into the changelog post. (this might be technically complex but is important)

7.  For `dsc topic sync`, I added `--yes` for non-interactive runs. OK to keep?

A: Yes, and shorten it to `-y` for convenience.

8. The test requirement says every command’s e2e test must send messages to Discourse. For commands like `list/add/import`, is it acceptable for the test to post a marker message as setup/verification even though the command itself is local-only?

A: If the command is local-only, the e2e test does not need to post to Discourse. It can verify functionality locally. Only commands that interact with Discourse need to send messages.
