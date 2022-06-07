use byteorder::{ReadBytesExt, LittleEndian};
use std::io::Read;
use std::io::{Error};
use std::fmt::{Display,Debug};
use std::fmt;

#[derive(Clone)]
pub struct Guid(pub [u8;16]);
impl Guid {
    pub fn new<R: Read>(mut reader: R) -> Result<Guid,Error> {
        let mut buffer = [0; 16];
        reader.read_exact(&mut buffer)?;
        Ok(
            Guid(buffer)
        )
    }

    pub fn to_string(&self) -> String {
        let mut slice: &[u8] = &self.0;
        format!(
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            slice.read_u32::<LittleEndian>().unwrap(),
            slice.read_u16::<LittleEndian>().unwrap(),
            slice.read_u16::<LittleEndian>().unwrap(),
            slice.read_u8().unwrap(),
            slice.read_u8().unwrap(),
            slice.read_u8().unwrap(),
            slice.read_u8().unwrap(),
            slice.read_u8().unwrap(),
            slice.read_u8().unwrap(),
            slice.read_u8().unwrap(),
            slice.read_u8().unwrap()
        )
    }
}
impl Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.to_string())
    }
}
impl Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.to_string())
    }
}
