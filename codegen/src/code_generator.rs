use crate::parser::data::ParsedData;
use std::path::PathBuf;

mod moves;
mod species;

pub struct CodeGenerator {
    output_dir: PathBuf,
}

impl CodeGenerator {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }

    pub fn generate(&self, data: &ParsedData) -> anyhow::Result<()> {
        let species_content = species::generate(data)?;
        std::fs::write(self.output_dir.join("species_id.rs"), species_content)?;

        let moves_content = moves::generate(data)?;
        std::fs::write(self.output_dir.join("move_id.rs"), moves_content)?;
        Ok(())
    }
}

pub fn to_pascal(s: &str) -> String {
    let result: String = s
        .split(['-', '_'])
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().to_string() + c.as_str(),
            }
        })
        .collect();

    if result.starts_with(|c: char| c.is_ascii_digit()) {
        format!("N{result}")
    } else {
        result
    }
}
