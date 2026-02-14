use crate::storage::pokemon::StoredPokemon;

#[derive(Debug, Default)]
pub struct StoredTeam([Option<StoredPokemon>; 6]);

impl StoredTeam {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all(&self) -> &[Option<StoredPokemon>; 6] {
        &self.0
    }

    pub fn get(&self, index: usize) -> Option<&StoredPokemon> {
        self.0.get(index).and_then(|o| o.as_ref())
    }

    pub fn add(&mut self, slot: usize, pokemon: StoredPokemon) {
        if slot < 6 {
            self.0[slot] = Some(pokemon);
        }
    }
}
