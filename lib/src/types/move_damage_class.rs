use strum_macros::{Display, EnumIter, FromRepr};

/// Source: https://github.com/PokeAPI/pokeapi/blob/master/data/v2/csv/move_damage_classes.csv
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter, Display, FromRepr)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemSize, mem_dbg::MemDbg))]
#[cfg_attr(feature = "mem_dbg", mem_size_flat)]
#[repr(u8)]
pub enum MoveDamageClass {
    Status = 1,
    Physical = 2,
    Special = 3,
}
