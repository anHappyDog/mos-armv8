const ENTRIES_PER_TABLE: usize = 512;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry(usize);

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; ENTRIES_PER_TABLE],
}

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }
}

impl PageTable {
    pub const fn empty() -> Self {
        Self {
            entries: [PageTableEntry::empty(); ENTRIES_PER_TABLE],
        }
    }
    pub fn map(
        &mut self,
        aligned_va: usize,
        aligned_pa: usize,
        len: usize,
        level: usize,
    ) -> Result<(), ()> {
        todo!()
    }

    pub fn unmap(&mut self, aligned_va: usize, len: usize, level: usize) {
        todo!()
    }

    pub fn translate(&self, va: usize) -> Option<usize> {
        todo!()
    }
}
