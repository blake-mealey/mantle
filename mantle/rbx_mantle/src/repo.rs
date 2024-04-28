use std::{path::Path, process::Command, str};

use anyhow::bail;

fn run_command(dir: &Path, command: &str) -> std::io::Result<std::process::Output> {
    if cfg!(target_os = "windows") {
        return Command::new("cmd")
            .current_dir(dir)
            .arg("/C")
            .arg(command)
            .output();
    } else {
        return Command::new("sh")
            .current_dir(dir)
            .arg("-c")
            .arg(command)
            .output();
    }
}

pub fn get_current_branch(project_path: &Path) -> anyhow::Result<String> {
    let output = run_command(project_path, "git symbolic-ref --short HEAD");
    let result = match output {
        Ok(v) => v,
        Err(e) => {
            bail!(
                "Unable to determine git branch. Are you in a git repository?\n\t{}",
                e
            )
        }
    };

    if !result.status.success() {
        bail!("Unable to determine git branch. Are you in a git repository?");
    }

    let current_branch = str::from_utf8(&result.stdout).unwrap().trim();
    if current_branch.is_empty() {
        bail!("Unable to determine git branch. Are you in a git repository?");
    }

    Ok(current_branch.to_owned())
}

pub fn match_branch(branch: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        let glob_pattern = glob::Pattern::new(pattern);
        if glob_pattern.is_ok() && glob_pattern.unwrap().matches(branch) {
            return true;
        }
    }
    false
}
