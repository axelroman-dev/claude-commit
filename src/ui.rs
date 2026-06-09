use crate::config::Config;
use colored::Colorize;
use dialoguer::{Input, Select};

pub fn print_banner() {
    println!("{}", "╔═══════════════════════════╗".cyan());
    println!("{}", "║      commit-ai 🦀          ║".cyan());
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

pub fn print_loading() {
    println!("{}", "🤖 Analizando diff con Claude Code...".cyan());
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗ Error:".red().bold(), msg.red());
}

pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg.green());
}

pub fn select_suggestion(suggestions: &[String]) -> Option<String> {
    if suggestions.is_empty() {
        print_error("No se generaron sugerencias");
        return None;
    }

    println!("{}", "💡 Sugerencias de commit:".yellow().bold());
    println!();

    let mut opciones = suggestions.to_vec();
    opciones.push("✗ Cancelar".to_string());

    let selection = Select::new()
        .with_prompt("Selecciona un mensaje (↑↓ para navegar, Enter para elegir)")
        .items(&opciones)
        .default(0)
        .interact();

    match selection {
        Ok(index) if index < suggestions.len() => Some(suggestions[index].clone()),
        _ => None,
    }
}

pub fn setup_wizard() -> Config {
    println!("{}", "⚙️  Configuración de commit-ai".cyan().bold());
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

    Config {
        language: idiomas[lang_idx].to_string(),
        commit_style: estilos[style_idx].to_string(),
        suggestions_count,
    }
}
