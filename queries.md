## Questions

1. Backup API endpoints: I implemented `POST /admin/backups.json`, `GET /admin/backups.json`, and `POST /admin/backups/{backup_path}/restore`. Are these the exact endpoints/params you want (e.g., `with_uploads` on create, restore URL shape)?

2. Completion scripts: the implementation plan calls for bash/zsh/fish completion scripts, but they are not present. Should I add generation to the repo now (e.g., via `clap` completions) and where should the scripts live?
