#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemSize, mem_dbg::MemDbg))]
#[cfg_attr(feature = "mem_dbg", mem_size_flat)]
pub struct Stats {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub sp_atk: u8,
    pub sp_def: u8,
    pub speed: u8,
}
