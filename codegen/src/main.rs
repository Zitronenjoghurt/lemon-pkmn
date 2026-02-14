use crate::code_generator::CodeGenerator;
use crate::data_generator::DataGenerator;
use crate::fetcher::Fetcher;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

mod code_generator;
mod data_generator;
mod fetcher;
mod parser;

#[tokio::main]
async fn main() {
    init_logging();

    let data_dir = PathBuf::from("./data");
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).unwrap();
    }
    Fetcher::new(&data_dir).fetch_all().await.unwrap();

    let parsed_data = parser::Parser::new(data_dir).parse_all().unwrap();

    let codegen_dir = PathBuf::from("./lib/src/data");
    CodeGenerator::new(codegen_dir)
        .generate(&parsed_data)
        .unwrap();

    let data_file = PathBuf::from("./data.bin");
    DataGenerator::new(&data_file)
        .generate(parsed_data)
        .unwrap();
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();
}
