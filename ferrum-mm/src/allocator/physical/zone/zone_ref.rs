use super::ZoneType;

#[derive(Clone, Copy)]
pub struct ZoneRef {
    pub node_id: usize,
    pub zone: ZoneType,
}
