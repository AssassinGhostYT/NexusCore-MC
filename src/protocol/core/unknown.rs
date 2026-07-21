use super::PacketDyn;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct UnknownPacket {
    pub id: u16,
    pub buf: Box<[u8]>,
}

impl PacketDyn for UnknownPacket {
    #[inline]
    fn id(&self) -> u16 {
        self.id
    }
}
