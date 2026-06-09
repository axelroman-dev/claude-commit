use std::process::Command;

pub struct GitDiff {
    pub content: String,
    pub files_changed: Vec<String>,
    pub is_empty: bool,
}

pub fn get_diff() -> Result<GitDiff, String> {
    // Primero intentamos diff de staged (git add ya hecho)
    let output = Command::new("git")
        .args(["diff", "--staged"])
        .output()
        .map_err(|_| "Error: git no encontrado o no estás en un repo".to_string())?;

    let mut content = String::from_utf8_lossy(&output.stdout).to_string();

    // Si no hay staged, usamos el diff normal (sin git add)
    if content.trim().is_empty() {
        let output = Command::new("git")
            .args(["diff"])
            .output()
            .map_err(|_| "Error ejecutando git diff".to_string())?;

        content = String::from_utf8_lossy(&output.stdout).to_string();
    }

    let is_empty = content.trim().is_empty();

    // Extraer nombres de archivos modificados
    let files_changed = extract_files(&content);

    Ok(GitDiff {
        content,
        files_changed,
        is_empty,
    })
}

// Saca los nombres de archivo del diff
fn extract_files(diff: &str) -> Vec<String> {
    diff.lines()
        .filter(|line| line.starts_with("diff --git"))
        .map(|line| {
            // "diff --git a/src/main.rs b/src/main.rs" -> "src/main.rs"
            line.split(" b/").last().unwrap_or("unknown").to_string()
        })
        .collect()
}

// Limita el diff a N caracteres para no exceder el contexto de Claude
pub fn truncate_diff(diff: &str, max_chars: usize) -> String {
    if diff.len() <= max_chars {
        return diff.to_string();
    }

    let truncated = &diff[..max_chars];
    format!("{}\n\n... [diff truncado por tamaño]", truncated)
}
