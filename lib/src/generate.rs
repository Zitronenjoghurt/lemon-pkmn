pub mod evs;
pub mod ivs;
pub mod level;
pub mod nature;
pub mod species_id;
pub mod stored_moves;
pub mod stored_pokemon;
pub mod version_group;

pub fn thread_rng() -> rand::rngs::ThreadRng {
    rand::rng()
}
