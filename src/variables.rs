use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::Deserialize;
use toml;
use anyhow::Result;

#[derive(Debug, Deserialize, Default)]
pub struct Variables {
    #[serde(flatten)]
    vars: HashMap<String, toml::Value>,
    #[serde(skip)]
    env_vars: Option<HashMap<String, toml::Value>>,
    #[serde(skip)]
    page_vars: Option<HashMap<String, toml::Value>>,
}

impl Variables {
    pub fn load(base_path: &Path) -> Result<Self> {
        let mut vars = Self::load_file(&base_path.join("variables.toml"))?;
        
        // Load environment-specific variables
        if cfg!(debug_assertions) {
            if let Ok(env_vars) = Self::load_file(&base_path.join("variables.dev.toml")) {
                vars.env_vars = Some(env_vars.vars);
            }
        } else {
            if let Ok(env_vars) = Self::load_file(&base_path.join("variables.prod.toml")) {
                vars.env_vars = Some(env_vars.vars);
            }
        }
        
        Ok(vars)
    }

    fn load_file(path: &Path) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn set_page_vars(&mut self, vars: HashMap<String, toml::Value>) {
        self.page_vars = Some(vars);
    }

    pub fn get(&self, key: &str) -> Option<&toml::Value> {
        // Check in order: page vars -> env vars -> global vars
        if let Some(page_vars) = &self.page_vars {
            if let Some(value) = page_vars.get(key) {
                return Some(value);
            }
        }

        if let Some(env_vars) = &self.env_vars {
            if let Some(value) = env_vars.get(key) {
                return Some(value);
            }
        }

        self.vars.get(key)
    }
}