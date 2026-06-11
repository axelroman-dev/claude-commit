use crate::config::Config;
use colored::Colorize;
use dialoguer::console::{style, Style};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, MultiSelect, Select};

fn theme() -> ColorfulTheme {
    ColorfulTheme {
        active_item_prefix: style("●".to_string()).for_stderr().cyan().bold(),
        inactive_item_prefix: style("○".to_string()).for_stderr().dim(),
        active_item_style: Style::new().for_stderr().cyan().bold(),
        checked_item_prefix: style("[x]".to_string()).for_stderr().green(),
        unchecked_item_prefix: style("[ ]".to_string()).for_stderr().dim(),
        ..ColorfulTheme::default()
    }
}

pub fn print_banner() {
    println!("{}", "╔═══════════════════════════╗".cyan());
    println!("{}", "║      claude-commit 🦀     ║".cyan());
    println!("{}", "╚═══════════════════════════╝".cyan());
    println!();
}

pub fn print_files(files: &[String]) {
    println!("{}", "📁 Archivos modificados:".yellow().bold());
    for file in files {
        println!("  {} {}", "→".dimmed(), file.white());
    }
    println!();
}

pub fn print_staged_files(files: &[String]) {
    println!("{}", "📦 Ya en stage:".green().bold());
    for file in files {
        println!("  {} {}", "✔".green(), file.white());
    }
    println!();
}

pub fn print_loading() {
    println!("{}", "🤖 Analizando diff con Claude Code...".cyan());
}

pub fn print_usage(input_tokens: u64, output_tokens: u64, cost_usd: f64) {
    println!(
        "{}  in: {} | out: {} | ${:.4}",
        "tokens".dimmed(),
        input_tokens.to_string().dimmed(),
        output_tokens.to_string().dimmed(),
        cost_usd,
    );
}

pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg.yellow());
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗ Error:".red().bold(), msg.red());
}

pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg.green());
}

pub enum SuggestionResult {
    Commit(String),
    Regenerate,
    Cancel,
}

pub fn select_suggestion(suggestions: &[String]) -> SuggestionResult {
    if suggestions.is_empty() {
        print_error("No se generaron sugerencias");
        return SuggestionResult::Cancel;
    }

    let mut opciones: Vec<String> = suggestions.to_vec();
    let regen_idx = opciones.len();
    opciones.push("↻ Regenerar sugerencias".to_string());

    loop {
        println!("{}", "💡 Sugerencias de commit:".yellow().bold());
        println!();

        let selection = Select::with_theme(&theme())
            .with_prompt("Selecciona un mensaje (↑↓ para navegar, Enter para elegir)")
            .report(false)
            .items(&opciones)
            .default(0)
            .interact();

        match selection {
            Ok(index) if index < suggestions.len() => {
                println!();
                match confirm_or_edit(&suggestions[index]) {
                    ConfirmResult::Confirm(msg) => return SuggestionResult::Commit(msg),
                    ConfirmResult::Back => {
                        println!();
                        continue;
                    }
                    ConfirmResult::Cancel => return SuggestionResult::Cancel,
                }
            }
            Ok(index) if index == regen_idx => return SuggestionResult::Regenerate,
            _ => return SuggestionResult::Cancel,
        }
    }
}

enum ConfirmResult {
    Confirm(String),
    Back,
    Cancel,
}

fn confirm_or_edit(message: &str) -> ConfirmResult {
    println!("{} {}", "→".dimmed(), message.bold());
    println!();

    let opciones = ["Confirmar", "Editar", "← Volver"];

    let selection = Select::with_theme(&theme())
        .with_prompt("¿Qué quieres hacer?")
        .report(false)
        .items(&opciones)
        .default(0)
        .interact();

    match selection {
        Ok(0) => ConfirmResult::Confirm(message.to_string()),
        Ok(1) => {
            let edited: Result<String, _> = Input::with_theme(&theme())
                .with_prompt("Edita el mensaje")
                .with_initial_text(message.to_string())
                .interact_text();

            match edited {
                Ok(text) => {
                    let trimmed = text.trim().to_string();
                    if trimmed.is_empty() {
                        ConfirmResult::Back
                    } else {
                        ConfirmResult::Confirm(trimmed)
                    }
                }
                Err(_) => ConfirmResult::Cancel,
            }
        }
        Ok(_) => ConfirmResult::Back,
        Err(_) => ConfirmResult::Cancel,
    }
}

pub fn select_files_to_stage(files: &[String]) -> Option<Vec<String>> {
    let selected = MultiSelect::with_theme(&theme())
        .with_prompt("Selecciona archivos a agregar (Space para marcar, Enter para continuar)")
        .items(files)
        .interact();

    match selected {
        Ok(indices) => Some(indices.iter().map(|&i| files[i].clone()).collect()),
        Err(_) => None,
    }
}

pub fn setup_wizard() -> Config {
    println!("{}", "⚙️  Configuración de claude-commit".cyan().bold());
    println!("{}", "──────────────────────────────".dimmed());
    println!();

    let idiomas = ["spanish", "english"];
    let lang_idx = Select::new()
        .with_prompt("🌎 Idioma de los commits")
        .items(&idiomas)
        .default(0)
        .interact()
        .unwrap_or(0);

    let estilos = ["conventional", "simple"];
    let style_idx = Select::new()
        .with_prompt("📝 Estilo de commits")
        .items(&estilos)
        .default(0)
        .interact()
        .unwrap_or(0);

    let count_str: String = Input::new()
        .with_prompt("🔢 Cuántas sugerencias generar (1-5)")
        .default("3".to_string())
        .interact_text()
        .unwrap_or("3".to_string());

    let suggestions_count: u8 = count_str.parse().unwrap_or(3).min(5).max(1);

    let max_length_str: String = Input::new()
        .with_prompt("📏 Longitud máxima del título del commit")
        .default("80".to_string())
        .interact_text()
        .unwrap_or("80".to_string());

    let max_title_length: u8 = max_length_str.parse().unwrap_or(80).max(20);

    Config {
        language: idiomas[lang_idx].to_string(),
        commit_style: estilos[style_idx].to_string(),
        suggestions_count,
        max_title_length,
    }
}
