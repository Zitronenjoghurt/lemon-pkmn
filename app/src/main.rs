use lemon_pkmn::data::species_id::SpeciesId;
use lemon_pkmn::data::Data;
use lemon_pkmn::generate::stored_pokemon::StoredPokemonGenerator;
use lemon_pkmn::generate::thread_rng;
use lemon_pkmn::types::version_group::VersionGroup;

fn main() {
    let data = Data::load_included().unwrap();

    let generator = StoredPokemonGenerator::default()
        .with_species_id(SpeciesId::Ampharos)
        .with_level(50)
        .with_version_group(VersionGroup::HeartgoldSoulsilver);

    let pokemon = generator.generate(&mut thread_rng(), &data).unwrap();
    println!("{pokemon:#?}")
}
