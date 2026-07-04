pub const ID_JIGSAW_STRUCTURE_DATA: u32 = 313;

#[derive(Debug, Clone)]
pub struct JigsawStructureData {}

impl JigsawStructureData {
    pub fn new() -> Self {
        JigsawStructureData {}
    }

    pub fn write(&self) -> Vec<u8> {
        // En Network NBT, un compound vacío (StructureData) ocupa exactamente 3 bytes:
        // 0x0a (TAG_Compound) + 0x00 (empty root name length as 1-byte varint) + 0x00 (TAG_End)
        vec![0x0a, 0x00, 0x00]
    }
}
