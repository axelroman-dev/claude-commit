use std::process::Command;

pub struct GitDiff {
    pub content: String,
    pub files_changed: Vec<String>,
    pub is_empty: bool,
}

pub fn get_staged_diff() -> Result<GitDiff, String> {
    let output = Command::new("git")
        .args(["diff", "--staged"])
        .output()
        .map_err(|_| "Error: git no encontrado o no estás en un repo".to_string())?;

    let content = String::from_utf8_lossy(&output.stdout).to_string();
    let is_empty = content.trim().is_empty();
    let files_changed = extract_files(&content);

    Ok(GitDiff { content, files_changed, is_empty })
}

pub fn get_unstaged_files() -> Vec<String> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    // Porcelain format: "XY filename"
    // X = staged status, Y = unstaged status
    // Show files where Y != ' ' (unstaged changes) or ?? (untracked)
    output
        .lines()
        .filter(|line| line.len() > 3)
        .filter(|line| {
            let y = line.chars().nth(1).unwrap_or(' ');
            let untracked = line.starts_with("??");
            y != ' ' || untracked
        })
        .map(|line| line[3..].to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn stage_files(files: &[String]) -> Result<(), String> {
    let output = Command::new("git")
        .arg("add")
        .args(files)
        .output()
        .map_err(|e| format!("Error ejecutando git add: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git add error: {}", stderr));
    }

    Ok(())
}

fn extract_files(diff: &str) -> Vec<String> {
    diff.lines()
        .filter(|line| line.starts_with("diff --git"))
        .map(|line| line.split(" b/").last().unwrap_or("unknown").to_string())
        .collect()
}

pub fn truncate_diff(diff: &str, max_chars: usize) -> String {
    if diff.len() <= max_chars {
        return diff.to_string();
    }

    let truncated = &diff[..max_chars];
    format!("{}\n\n... [diff truncado por tamaño]", truncated)
}
