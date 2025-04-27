use regex::Regex;
use lazy_static::lazy_static;
use crate::variables::Variables;
use anyhow::Result;

lazy_static! {
    static ref VAR_REGEX: Regex = Regex::new(r"@\{var\([\"']([^\"']+)[\"']\)\}").unwrap();
}

pub fn process_macros(content: &str, vars: &Variables) -> Result<String> {
    let mut result = content.to_string();
    
    // Process variable macros
    result = VAR_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let var_name = &caps[1];
        if let Some(value) = vars.get(var_name) {
            value.to_string()
        } else {
            log::warn!("Variable '{}' not found", var_name);
            format!("@{{var(\"{var_name}\")}}")
        }
    }).to_string();

    Ok(result)
}