use std::collections::HashMap;
use std::path::Path;
use std::fs;
use serde::Deserialize;
use toml;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VAR_REGEX: Regex = Regex::new(r#"@\{var\(["']([^"']+)["']\)\}"#).unwrap();
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Variables {
    #[serde(flatten)]
    vars: HashMap<String, toml::Value>,
    #[serde(skip)]
    env_vars: Option<HashMap<String, toml::Value>>,
    #[serde(skip)]
    page_vars: Option<HashMap<String, toml::Value>>,
}

impl Variables {
    pub fn load(config_path: &Path) -> Result<Self> {
        let mut vars = Self::load_file(config_path)?;
        
        // Load environment-specific variables using same directory as config
        let base_dir = config_path.parent().unwrap_or(Path::new(""));
        let env_file = if cfg!(debug_assertions) {
            base_dir.join("variables.dev.toml")
        } else {
            base_dir.join("variables.prod.toml")
        };
        
        if let Ok(env_vars) = Self::load_file(&env_file) {
            vars.env_vars = Some(env_vars.vars);
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

    pub fn substitute(&self, content: &str) -> String {
        VAR_REGEX.replace_all(content, |caps: &regex::Captures| {
            let var_name = &caps[1];
            if let Some(value) = self.get(var_name) {
                value.to_string()
            } else {
                log::warn!("Variable '{}' not found", var_name);
                format!("@{{var(\"{var_name}\")}}")
            }
        }).to_string()
    }
}

pub fn load_variables(config_path: &Path) -> Result<Variables> {
    Variables::load(config_path)
}