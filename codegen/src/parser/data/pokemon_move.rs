use crate::parser::csv::CsvData;
use crate::parser::data::DataRecord;
use anyhow::Context;
use lemon_pkmn::types::move_damage_class::MoveDamageClass;
use lemon_pkmn::types::move_target::MoveTarget;
use lemon_pkmn::types::pokemon_type::PokemonType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PokemonMoveRecord {
    pub id: u16,
    pub identifier: String,
    pub pokemon_type: PokemonType,
    pub power: u8,
    pub pp: u8,
    pub accuracy: u8,
    pub priority: i8,
    pub target: MoveTarget,
    pub damage_class: MoveDamageClass,
}

impl DataRecord for PokemonMoveRecord {
    fn parse(csv_data: &CsvData) -> anyhow::Result<Vec<Self>> {
        let mut moves = HashMap::new();

        for csv_record in &csv_data.moves {
            let move_record = PokemonMoveRecord {
                id: csv_record.id,
                identifier: csv_record.identifier.to_string(),
                pokemon_type: PokemonType::from_repr(csv_record.type_id)
                    .context(format!("Invalid type_id: {}", csv_record.type_id))?,
                power: csv_record.power.unwrap_or(0),
                pp: csv_record.pp.unwrap_or(0),
                accuracy: csv_record.accuracy.unwrap_or(0),
                priority: csv_record.priority,
                target: MoveTarget::from_repr(csv_record.target_id)
                    .context(format!("Invalid target_id: {}", csv_record.target_id))?,
                damage_class: MoveDamageClass::from_repr(csv_record.damage_class_id).context(
                    format!("Invalid damage_class_id: {}", csv_record.damage_class_id),
                )?,
            };
            moves.insert(csv_record.id, move_record);
        }

        let mut moves: Vec<_> = moves.into_values().collect();
        moves.sort_by_key(|move_record| move_record.id);
        Ok(moves)
    }
}
