# Implementation Plan

## Source Spec
- Implement every command and flag described in `spec/spec.md`.
- Track decisions from `spec/queries.md`.

## CLI
- Build the `dsc` CLI in Rust with standard Unix-style syntax.
- Ensure every flag has a short alias (e.g., `--format` -> `-f`).
- Use `--discourse`/`-d` to select a target install when multiple are configured.

## Update Workflow
- Use SSH config entries (via `ssh <name>`) for updates; no SSH credentials in `dsc.toml`.
- Collect version/reclaimed-space data during updates and include it in the changelog post.

## Testing
- Each Discourse-interacting command must have an end-to-end test that posts a marker and verifies it on the forum.
- Local-only commands should be tested locally without touching Discourse.
- Test credentials/config will be provided in `testdsc.toml`.

## Configuration Files
- Keep a version-controlled `dsc.example.toml`.
- Ignore real `dsc.toml` and `testdsc.toml`.

## Code Quality
- Add Rustdoc comments to all public structs and functions.
- Add concise inline comments only where logic is non-obvious.
