use reqwest::Client;
use std::path::{Path, PathBuf};

const POKEAPI_BASE_URL: &str =
    "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv";
const POKEAPI_REQUIRED: &[&str] = &[
    "pokemon",
    "pokemon_forms",
    "pokemon_species",
    "pokemon_stats",
];

pub struct Fetcher {
    client: Client,
    data_dir: PathBuf,
}

impl Fetcher {
    pub fn new(data_dir: &Path) -> Self {
        Self {
            client: Client::new(),
            data_dir: data_dir.to_owned(),
        }
    }

    pub async fn fetch_all(&self) -> anyhow::Result<()> {
        for file_name in POKEAPI_REQUIRED {
            let path = self.data_dir.join(format!("{file_name}.csv"));
            if path.exists() {
                tracing::info!("Skip downloading existing file: {}", path.display());
                continue;
            }

            let url = format!("{POKEAPI_BASE_URL}/{file_name}.csv");
            tracing::info!("Fetching: {}", url);
            let response = self.client.get(&url).send().await?;

            let data = response.bytes().await?;
            std::fs::write(&path, data)?;
            tracing::info!("Fetched data file: {}", path.display());
        }

        Ok(())
    }
}
