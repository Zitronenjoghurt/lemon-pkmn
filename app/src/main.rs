use lemon_pkmn::data::Data;
use lemon_pkmn::generate::stored_pokemon::StoredPokemonGenerator;
use lemon_pkmn::generate::thread_rng;

fn main() {
    let data = Data::load_included().unwrap();
    let generator = StoredPokemonGenerator::default();
    let pokemon = generator.generate(&mut thread_rng(), &data).unwrap();
    println!("{pokemon:#?}")
}
