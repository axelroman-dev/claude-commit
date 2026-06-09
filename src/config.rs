use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub language: String,
    pub commit_style: String,
    pub suggestions_count: u8,
    pub max_title_length: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            language: "english".to_string(),
            commit_style: "conventional".to_string(),
            suggestions_count: 3,
            max_title_length: 80,
        }
    }
}

impl Config {
    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().expect("No se pudo encontrar directorio de config");
        path.push("claude-commit");
        path.push("config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();

        if !path.exists() {
            return Config::default();
        }

        let content = fs::read_to_string(&path).expect("Error leyendo config");
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) {
        let path = Self::config_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Error creando directorio de config");
        }

        let content = toml::to_string(self).expect("Error serializando config");
        fs::write(&path, content).expect("Error guardando config");
    }
}
