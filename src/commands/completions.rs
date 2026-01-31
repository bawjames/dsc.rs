use crate::cli::{Cli, CompletionShell};
use crate::utils::ensure_dir;
use anyhow::{Context, Result};
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::fs;
use std::io;
use std::path::Path;

pub fn write_completions(shell: CompletionShell, dir: Option<&Path>) -> Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    match dir {
        Some(dir) => {
            ensure_dir(dir)?;
            let filename = match shell {
                CompletionShell::Bash => "dsc.bash",
                CompletionShell::Zsh => "_dsc",
                CompletionShell::Fish => "dsc.fish",
            };
            let path = dir.join(filename);
            let generator: Shell = shell.into();
            if matches!(shell, CompletionShell::Zsh) {
                let mut buffer = Vec::new();
                generate(generator, &mut cmd, name, &mut buffer);
                let content = String::from_utf8(buffer).context("decoding zsh completions")?;
                let content = inject_zsh_sort_style(content);
                fs::write(&path, content).with_context(|| format!("writing {}", path.display()))?;
            } else {
                let mut file = fs::File::create(&path)
                    .with_context(|| format!("creating {}", path.display()))?;
                generate(generator, &mut cmd, name, &mut file);
            }
            println!("{}", path.display());
        }
        None => {
            let generator: Shell = shell.into();
            if matches!(shell, CompletionShell::Zsh) {
                let mut buffer = Vec::new();
                generate(generator, &mut cmd, name, &mut buffer);
                let content = String::from_utf8(buffer).context("decoding zsh completions")?;
                let content = inject_zsh_sort_style(content);
                print!("{}", content);
            } else {
                let mut stdout = io::stdout();
                generate(generator, &mut cmd, name, &mut stdout);
            }
        }
    }
    Ok(())
}

fn inject_zsh_sort_style(mut content: String) -> String {
    let style = "zstyle ':completion:*:dsc:*' sort false";
    if content.contains(style) {
        return content;
    }
    let marker = "autoload -U is-at-least\n";
    if let Some(pos) = content.find(marker) {
        let insert_at = pos + marker.len();
        content.insert_str(insert_at, &format!("\n{}\n", style));
        return content;
    }
    format!("{}\n\n{}", style, content)
}
