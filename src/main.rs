mod claude;
mod config;
mod git;
mod ui;

use clap::{Parser, Subcommand};
use config::Config;
use std::process::Command;

#[derive(Parser)]
#[command(name = "claude-commit")]
#[command(about = "Genera sugerencias de commits con IA 🦀")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generar sugerencias de commit (default)
    Suggest,
    /// Configurar claude-commit
    Config {
        #[arg(long)]
        language: Option<String>,
        #[arg(long)]
        style: Option<String>,
        #[arg(long)]
        count: Option<u8>,
        #[arg(long)]
        max_length: Option<u8>,
    },
    /// Ver configuración actual
    Show,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Config {
            language,
            style,
            count,
            max_length,
        }) => {
            run_config(language, style, count, max_length);
        }
        Some(Commands::Show) => {
            run_show();
        }
        Some(Commands::Suggest) | None => {
            run_suggest();
        }
    }
}

fn run_show() {
    let config = Config::load();
    println!("⚙️  Configuración actual:");
    println!("  idioma:       {}", config.language);
    println!("  estilo:       {}", config.commit_style);
    println!("  sugerencias:  {}", config.suggestions_count);
    println!("  max. título:  {} chars", config.max_title_length);
}

fn run_config(
    language: Option<String>,
    style: Option<String>,
    count: Option<u8>,
    max_length: Option<u8>,
) {
    if language.is_none() && style.is_none() && count.is_none() && max_length.is_none() {
        let config = ui::setup_wizard();
        config.save();
        ui::print_success("Configuración guardada!");
        return;
    }

    let mut config = Config::load();

    if let Some(l) = language {
        config.language = l;
    }
    if let Some(s) = style {
        config.commit_style = s;
    }
    if let Some(c) = count {
        config.suggestions_count = c;
    }
    if let Some(m) = max_length {
        config.max_title_length = m;
    }

    config.save();
    ui::print_success("Configuración actualizada!");
}

fn run_suggest() {
    ui::print_banner();

    // Verificar que claude CLI está instalado
    if !claude::check_installed() {
        ui::print_error("Claude Code CLI no está instalado o no está en el PATH.");
        println!("  Instálalo desde: https://claude.ai/code");
        println!("  Luego inicia sesión con: claude login");
        return;
    }

    let config = Config::load();

    let diff = match git::get_staged_diff() {
        Ok(d) => d,
        Err(e) => {
            ui::print_error(&e);
            return;
        }
    };

    let diff = if diff.is_empty {
        let unstaged = git::get_unstaged_files();

        if unstaged.is_empty() {
            ui::print_error("No hay cambios en el repositorio.");
            println!("  Haz cambios primero.");
            return;
        }

        println!();
        ui::print_warning("No hay cambios en stage.");
        println!();

        let files_to_stage = match ui::select_files_to_stage(&unstaged) {
            Some(f) => f,
            None => {
                println!("Cancelado.");
                return;
            }
        };

        if files_to_stage.is_empty() {
            ui::print_error("No seleccionaste ningún archivo.");
            return;
        }

        if let Err(e) = git::stage_files(&files_to_stage) {
            ui::print_error(&e);
            return;
        }

        println!();
        match git::get_staged_diff() {
            Ok(d) => d,
            Err(e) => {
                ui::print_error(&e);
                return;
            }
        }
    } else {
        let unstaged = git::get_unstaged_files();

        if unstaged.is_empty() {
            diff
        } else {
            println!();
            ui::print_warning(&format!("{} archivo(s) con cambios sin stagear:", unstaged.len()));
            println!();

            match ui::select_files_to_stage(&unstaged) {
                Some(files_to_stage) if !files_to_stage.is_empty() => {
                    if let Err(e) = git::stage_files(&files_to_stage) {
                        ui::print_error(&e);
                        return;
                    }
                    println!();
                    match git::get_staged_diff() {
                        Ok(d) => d,
                        Err(e) => {
                            ui::print_error(&e);
                            return;
                        }
                    }
                }
                Some(_) => diff,
                None => {
                    println!("Cancelado.");
                    return;
                }
            }
        }
    };

    ui::print_files(&diff.files_changed);

    let diff_content = git::truncate_diff(&diff.content, 8000);

    ui::print_loading();
    let (suggestions, usage) = match claude::get_suggestions(&diff_content, &config) {
        Ok(s) => s,
        Err(e) => {
            ui::print_error(&e);
            return;
        }
    };

    ui::print_usage(usage.input_tokens, usage.output_tokens, usage.cost_usd);
    println!();

    let selected = ui::select_suggestion(&suggestions);

    match selected {
        Some(msg) => {
            println!();
            let output = Command::new("git").args(["commit", "-m", &msg]).output();

            match output {
                Ok(o) if o.status.success() => {
                    ui::print_success(&format!("Commit creado: {}", msg));
                }
                Ok(o) => {
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    ui::print_error(&format!("Git error: {}", stderr));
                }
                Err(e) => {
                    ui::print_error(&format!("Error ejecutando git: {}", e));
                }
            }
        }
        None => {
            println!("Cancelado.");
        }
    }
}
