use crate::fetcher::Fetcher;
use crate::generator::Generator;
use lemon_pkmn::data::print_data_size;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

mod fetcher;
mod generator;
mod parser;

#[tokio::main]
async fn main() {
    init_logging();

    println!("Previous data size:");
    print_data_size();

    let data_dir = PathBuf::from("./data");
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).unwrap();
    }
    Fetcher::new(&data_dir).fetch_all().await.unwrap();

    let parsed_data = parser::Parser::new(data_dir).parse_all().unwrap();

    let output_dir = PathBuf::from("./lib/src/data");
    Generator::new(output_dir).generate(&parsed_data).unwrap();
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();
}
