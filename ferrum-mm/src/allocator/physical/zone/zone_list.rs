use super::zone_ref::ZoneRef;
use super::{ZoneType, MAX_ZONELIST_ENTRIES};

pub struct ZoneList {
    refs: [ZoneRef; MAX_ZONELIST_ENTRIES],
    len: usize,
}

impl ZoneList {
    pub const fn empty() -> Self {
        const EMPTY_REF: ZoneRef = ZoneRef { node_id: 0, zone: ZoneType::Device };
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
