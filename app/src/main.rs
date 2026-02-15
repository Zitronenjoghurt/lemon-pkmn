use lemon_pkmn::battle::types::action::BattleAction;
use lemon_pkmn::battle::types::target::BattleTargetSingle;
use lemon_pkmn::battle::Battle;
use lemon_pkmn::data::move_id::MoveId;
use lemon_pkmn::data::Data;
use lemon_pkmn::generate::stored_pokemon::StoredPokemonGenerator;
use lemon_pkmn::generate::thread_rng;
use lemon_pkmn::storage::team::StoredTeam;
use std::sync::Arc;

fn main() {
    let data = Arc::new(Data::load_included().unwrap());
    let generator = StoredPokemonGenerator::default().moves(|g| g.specific(&[MoveId::Pound]));

    let pokemon_a = generator.generate(&mut thread_rng(), &data).unwrap();
    let pokemon_b = generator.generate(&mut thread_rng(), &data).unwrap();
    println!("{pokemon_a:#?}");
    println!("{pokemon_b:#?}");

    let mut team_a = StoredTeam::new();
    team_a.add(0, pokemon_a);
    let mut team_b = StoredTeam::new();
    team_b.add(0, pokemon_b);

    let mut battle = Battle::builder(&data)
        .add_team_a(&team_a)
        .unwrap()
        .add_team_b(&team_b)
        .unwrap()
        .build()
        .unwrap();

    battle
        .queue_action(BattleAction::use_move(
            BattleTargetSingle::new_a(0),
            BattleTargetSingle::new_b(0),
            0,
        ))
        .unwrap();
    battle
        .queue_action(BattleAction::use_move(
            BattleTargetSingle::new_b(0),
            BattleTargetSingle::new_a(0),
            0,
        ))
        .unwrap();
    println!("{:#?}", battle.resolve_turn().unwrap());
}
