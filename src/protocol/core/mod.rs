use std::any::Any;
use std::fmt::Debug;
use std::io::{Read, Write};

pub mod endian;
pub mod error;
pub mod header;
pub mod sub_client;
pub mod types;
pub mod meta;
pub mod unknown;

pub use endian::*;
pub use header::*;
pub use meta::*;
pub use unknown::*;


pub trait ProtoCodec: Sized {
    fn serialize<W: Write>(&self, stream: &mut W) -> Result<(), error::ProtoCodecError>;
    fn deserialize<R: Read>(stream: &mut R) -> Result<Self, error::ProtoCodecError>;
    fn size_hint(&self) -> usize;
}

pub trait Packet: ProtoCodec + Debug + Send + Sync + Any + 'static {
    const ID: u16;
}

pub trait PacketDyn: Debug + Send + Sync + Any + 'static {
    fn id(&self) -> u16;
}

impl<T: Packet> PacketDyn for T {
    #[inline]
    fn id(&self) -> u16 {
        T::ID
    }
}

pub trait Packets: Sized {
    fn serialize<W: Write>(
        &self,
        header: &PacketHeader,
        stream: &mut W,
    ) -> Result<(), error::PacketCodecError>;

    fn deserialize<R: Read>(stream: &mut R) -> Result<(Self, PacketHeader), error::PacketCodecError>;

    fn size_hint(&self, header: &PacketHeader) -> usize;

    fn id(&self) -> u16;
}
