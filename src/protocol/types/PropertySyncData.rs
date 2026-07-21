use crate::protocol::varint::write_varu32;

#[derive(Clone, Debug)]
pub struct IntEntry {
    pub property_index: u32,
    pub data: f32,
}

#[derive(Clone, Debug)]
pub struct FloatEntry {
    pub property_index: u32,
    pub data: i32,
}

#[derive(Clone, Debug)]
pub struct PropertySyncData {
    pub int_entries_list: Vec<IntEntry>,
    pub float_entries_list: Vec<FloatEntry>,
}

impl PropertySyncData {
    pub fn write(&self, buf: &mut Vec<u8>) {
        write_varu32(buf, self.int_entries_list.len() as u32);
        write_varu32(buf, self.float_entries_list.len() as u32);
    }
}
