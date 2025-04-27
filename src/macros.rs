use regex::Regex;
use lazy_static::lazy_static;
use crate::variables::Variables;

lazy_static! {
    static ref VAR_REGEX: Regex = Regex::new(r#"@\{var\(["']([^"']+)["']\)\}"#).unwrap();
}

pub struct MacroProcessor {
    variables: Option<Variables>
}

impl MacroProcessor {
    pub fn new() -> Self {
        Self {
            variables: None
        }
    }

    pub fn with_variables(mut self, vars: Variables) -> Self {
        self.variables = Some(vars);
        self
    }

    pub fn process(&self, content: &str) -> String {
        if let Some(vars) = &self.variables {
            VAR_REGEX.replace_all(content, |caps: &regex::Captures| {
                let var_name = &caps[1];
                if let Some(value) = vars.get(var_name) {
                    value.to_string()
                } else {
                    log::warn!("Variable '{}' not found", var_name);
                    format!("@{{var(\"{var_name}\")}}")
                }
            }).to_string()
        } else {
            content.to_string()
        }
    }
}