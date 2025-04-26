use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn generate_html(content: &str, components_dir: &str, cache: &mut HashMap<String, String>, visited: &mut HashSet<String>) -> String {
    let mut output = content.to_string();

    while let Some(start) = output.find("{{") {
        if let Some(end) = output[start..].find("}}") {
            let end = start + end + 2;
            let placeholder = &output[start..end];
            let component_name = placeholder[2..placeholder.len() - 2].trim();

            if visited.contains(component_name) {
                eprintln!("Error: Circular dependency detected for component '{}'.", component_name);
                return format!("<!-- Error: Circular dependency detected for component '{}' -->", component_name);
            }

            let component_content = match cache.get(component_name) {
                Some(cached) => cached.clone(),
                None => {
                    let component_path = Path::new(components_dir).join(format!("{}.html", component_name));
                    if component_path.exists() {
                        match fs::read_to_string(&component_path) {
                            Ok(content) => {
                                cache.insert(component_name.to_string(), content.clone());
                                content
                            }
                            Err(err) => {
                                eprintln!("Error reading component '{}': {}", component_name, err);
                                format!("<!-- Error reading component '{}' -->", component_name)
                            }
                        }
                    } else {
                        eprintln!("Warning: Component '{}' not found.", component_name);
                        format!("<!-- Warning: Component '{}' not found -->", component_name)
                    }
                }
            };

            visited.insert(component_name.to_string());

            let processed_content = generate_html(&component_content, components_dir, cache, visited);

            visited.remove(component_name);

            output = output.replacen(placeholder, &processed_content, 1);
        } else {
            eprintln!("Error: Unmatched '{{' in content.");
            return "<!-- Error: Unmatched '{{' in content -->".to_string();
        }
    }

    output
}

fn main() {
    let input_dir = "content";
    let output_dir = "output";
    let components_dir = "components";

    if !Path::new(output_dir).exists() {
        if let Err(err) = fs::create_dir(output_dir) {
            eprintln!("Error creating output directory: {}", err);
            return;
        }
    }

    let mut cache = HashMap::new();

    match fs::read_dir(input_dir) {
        Ok(entries) => {
            let entries: Vec<_> = entries.filter_map(Result::ok).collect(); // Collect entries upfront

            entries.iter().for_each(|entry| { // Process files in parallel
                let path = entry.path();

                if path.is_file() {
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            let mut visited = HashSet::new();
                            let output_content = generate_html(&content, components_dir, &mut cache, &mut visited);

                            let output_path = Path::new(output_dir).join(path.file_name().unwrap());
                            if let Err(err) = fs::write(output_path, output_content) {
                                eprintln!("Error writing output file: {}", err);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading file '{}': {}", path.display(), err);
                        }
                    }
                }
            });
        }
        Err(err) => {
            eprintln!("Error reading input directory '{}': {}", input_dir, err);
        }
    }
}
