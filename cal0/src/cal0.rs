use binread::{BinRead, NullString};
use std::io::{Read, Seek, SeekFrom, self};
use sha2::{Sha256, Digest};
use crate::crc16;
use std::convert::TryInto;

#[derive(Debug, BinRead)]
#[br(magic = b"CAL0", little)]
pub struct Cal0Header {
    pub version: u32,
    pub body_size: u32,
    pub model: u16,
    pub update_cnt: u16,

    #[br(pad_before(0xe))]
    pub header_crc: u16,
    
    pub body_hash: [u8; 0x20],
}

#[derive(Debug, BinRead)]
pub struct WlanCountryCode {
    raw: NullString,

}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Cal0Body {
    #[br(pad_size_to = 0x1e)]
    configuration_id1: NullString,

    #[br(seek_before = SeekFrom::Start(0x80))]
    wlan_country_codes_num: u32,

    wlan_country_codes_last_index: u32,

    #[br(count = wlan_country_codes_num)]
    wlan_country_codes: Vec<WlanCountryCode>,

    #[br(seek_before = SeekFrom::Start(0x250))]
    serial_number: [u8; 0x18],

    #[br(seek_before = SeekFrom::Start(0xad0), pad_size_to = 0x10)]
    ssl_certificate_size: u32,

    #[br(count = ssl_certificate_size, pad_size_to = 0x800)]
    ssl_cert: Vec<u8>
}

#[derive(Debug)]
pub struct Cal0<R: Read + Seek> {
    reader: R,
    pub header: Cal0Header,
    pub body: Cal0Body
}

impl<R: Read + Seek> Cal0<R> {
    pub fn from(mut reader: R, biskey: Option<String>) -> io::Result<Self> {
        let mut magic: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut magic).unwrap();

        if &magic != b"CAL0" {
            if let Some(biskey) = biskey {
                /* read from reader, decrypt with biskey, reader = dec_buf */
                
                /* return Self::from(dec_buf, biskey, ...) */
                unimplemented!();
            }
            else {
                /* return error that prodinfo needs to be in plaintext */
                unimplemented!();
            }
        }

        reader.seek(SeekFrom::Start(0))?;

        let header = Cal0Header::read(&mut reader).unwrap();
        let body = Cal0Body::read(&mut reader).unwrap();

        Ok(Cal0 { reader, header, body })
    }

    fn is_data_nulled(data: &[u8]) -> bool {
        data.iter().filter(|x| **x != 0x00).count() == 0
    }

    pub fn is_ssl_cert_nulled(&self) -> bool {
        Self::is_data_nulled(self.body.ssl_cert.as_slice())
    }

    pub fn is_body_hash_valid(&mut self) -> bool {
        let mut body_buf = vec![0; self.header.body_size as usize];
        self.reader.seek(SeekFrom::Start(0x40)).unwrap();
        self.reader.read_exact(&mut body_buf).unwrap();

        Sha256::digest(&body_buf) == self.header.body_hash.try_into().unwrap()
    }
}
