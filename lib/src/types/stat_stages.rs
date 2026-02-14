#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemSize, mem_dbg::MemDbg))]
#[cfg_attr(feature = "mem_dbg", mem_size_flat)]
pub struct StatStages {
    pub hp: i8,
    pub atk: i8,
    pub def: i8,
    pub sp_atk: i8,
    pub sp_def: i8,
    pub speed: i8,
}
