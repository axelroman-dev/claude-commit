use crate::config::Config;
use std::process::Command;

pub fn check_installed() -> bool {
    Command::new("claude")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn get_suggestions(diff: &str, config: &Config) -> Result<Vec<String>, String> {
    let prompt = build_prompt(diff, config);

    let output = Command::new("claude")
        .arg("--print")
        .arg(&prompt)
        .output()
        .map_err(|e| format!("Error ejecutando claude CLI: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("claude CLI error: {}", stderr));
    }

    let text = String::from_utf8_lossy(&output.stdout).to_string();
    let suggestions = parse_suggestions(&text);

    if suggestions.is_empty() {
        return Err("No se pudieron parsear sugerencias de la respuesta".to_string());
    }

    Ok(suggestions)
}

fn build_prompt(diff: &str, config: &Config) -> String {
    let lang_instruction = match config.language.as_str() {
        "spanish" => "Responde en español.",
        _ => "Respond in English.",
    };

    let style_instruction = match config.commit_style.as_str() {
        "conventional" => {
            "Usa el formato Conventional Commits: tipo(scope): descripción\n\
             Tipos válidos: feat, fix, docs, style, refactor, test, chore"
        }
        _ => "Usa un mensaje simple y descriptivo.",
    };

    format!(
        "Analiza este git diff y genera exactamente {} sugerencias de commit messages.\n\
         {}\n\
         {}\n\n\
         Reglas:\n\
         - Cada sugerencia en una línea separada\n\
         - Numeradas: 1. 2. 3.\n\
         - Máximo 72 caracteres por mensaje\n\
         - Solo los mensajes, sin explicaciones extra\n\n\
         Git diff:\n\
```\n{}\n```",
        config.suggestions_count, lang_instruction, style_instruction, diff
    )
}

fn parse_suggestions(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| {
            line.len() > 3 && line.chars().next().map(|c| c.is_numeric()).unwrap_or(false)
        })
        .map(|line| {
            line.splitn(2, ". ")
                .nth(1)
                .unwrap_or(line)
                .trim()
                .to_string()
        })
        .collect()
}
