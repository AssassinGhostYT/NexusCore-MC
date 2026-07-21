use crate::protocol::error::PResult;
use crate::protocol::varint::write_varu32;

pub struct ResourcePacksInfo {
    pub must_accept: bool,
    pub has_addons: bool,
    pub has_scripts: bool,
}

impl ResourcePacksInfo {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        buf.push(if self.must_accept { 1 } else { 0 });
        buf.push(if self.has_addons { 1 } else { 0 });
        buf.push(if self.has_scripts { 1 } else { 0 });
        buf.push(0);
        write_varu32(&mut buf, 0);
        write_varu32(&mut buf, 0);
        Ok(buf)
    }
}
