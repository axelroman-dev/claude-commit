use crate::config::Config;
use serde::Deserialize;
use std::process::Command;

pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cost_usd: f64,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    result: Option<String>,
    total_cost_usd: Option<f64>,
    usage: Option<ClaudeUsage>,
}

#[derive(Deserialize)]
struct ClaudeUsage {
    input_tokens: Option<u64>,
    cache_read_input_tokens: Option<u64>,
    output_tokens: Option<u64>,
}

pub fn check_installed() -> bool {
    Command::new("claude")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn get_suggestions(diff: &str, config: &Config) -> Result<(Vec<String>, Usage), String> {
    let prompt = build_prompt(diff, config);

    let output = Command::new("claude")
        .args(["--print", "--output-format", "json"])
        .arg(&prompt)
        .output()
        .map_err(|e| format!("Error ejecutando claude CLI: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("claude CLI error: {}", stderr));
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let response: ClaudeResponse = serde_json::from_str(&raw)
        .map_err(|e| format!("Error parseando respuesta de claude: {}", e))?;

    let text = response.result.unwrap_or_default();

    let usage = response.usage.as_ref();
    let input_tokens = usage
        .and_then(|u| u.input_tokens)
        .unwrap_or(0)
        + usage
            .and_then(|u| u.cache_read_input_tokens)
            .unwrap_or(0);
    let output_tokens = usage.and_then(|u| u.output_tokens).unwrap_or(0);

    let usage = Usage {
        input_tokens,
        output_tokens,
        cost_usd: response.total_cost_usd.unwrap_or(0.0),
    };

    let suggestions = parse_suggestions(&text);

    if suggestions.is_empty() {
        return Err("No se pudieron parsear sugerencias de la respuesta".to_string());
    }

    Ok((suggestions, usage))
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
         - Máximo {} caracteres por mensaje\n\
         - Solo los mensajes, sin explicaciones extra\n\n\
         Git diff:\n\
```\n{}\n```",
        config.suggestions_count, lang_instruction, style_instruction, config.max_title_length, diff
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
                .trim_matches('"')
                .trim_matches('\'')
                .trim_matches('`')
                .trim()
                .to_string()
        })
        .collect()
}
