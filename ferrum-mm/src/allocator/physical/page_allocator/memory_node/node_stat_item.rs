#[derive(Clone, Copy)]
#[repr(usize)]
pub enum NodeStatItem {
    InactiveAnon,
    ActiveAnon,
    InactiveFile,
    ActiveFile,
    Unevictable,
    SlabReclaimable,
    SlabUnreclaimable,
    IsolatedAnon,
    IsolatedFile,
    AnonMapped,
    FileMapped,
    FilePages,
    FileDirty,
    Writeback,
    Shmem,
    PageTable,
    KernelMiscReclaimable,
}

pub const NR_VM_NODE_STAT_ITEMS: usize = 17;
