use crate::storage::pokemon::StoredPokemon;

#[derive(Debug, Default)]
pub struct StoredTeam([Option<StoredPokemon>; 6]);

impl StoredTeam {
    pub fn all(&self) -> &[Option<StoredPokemon>; 6] {
        &self.0
    }

    pub fn get(&self, index: usize) -> Option<&StoredPokemon> {
        self.0.get(index).and_then(|o| o.as_ref())
    }
}
