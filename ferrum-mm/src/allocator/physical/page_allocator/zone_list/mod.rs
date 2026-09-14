pub mod zone_list_type;
pub mod zone_ref;

use crate::allocator::physical::page_allocator::constants::MAX_ZONELIST_ENTRIES;
use crate::allocator::physical::page_allocator::zone_type::ZoneType;
use zone_ref::ZoneRef;

pub struct ZoneList {
    refs: [ZoneRef; MAX_ZONELIST_ENTRIES],
    len: usize,
}

impl ZoneList {
    pub const fn empty() -> Self {
        const EMPTY_REF: ZoneRef = ZoneRef::new(0, ZoneType::Device);
        Self {
            refs: [EMPTY_REF; MAX_ZONELIST_ENTRIES],
            len: 0,
        }
    }

    pub fn push(&mut self, zoneref: ZoneRef) {
        if self.len < MAX_ZONELIST_ENTRIES {
            self.refs[self.len] = zoneref;
            self.len += 1;
        }
    }

    pub fn entries(&self) -> &[ZoneRef] {
        &self.refs[..self.len]
    }
}
