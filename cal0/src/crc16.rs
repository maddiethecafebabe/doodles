const TABLE: [u16; 0x10] = [
    0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401,
    0xA001, 0x6C00, 0x7800, 0xB401, 0x5000, 0x9C01, 0x8801, 0x4400
];

pub fn get_crc_16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0x55aa;

    for byte in data.iter() {
        let r = TABLE[(crc & 0xf) as usize];
        crc = (crc >> 4) & 0x0fff;
        crc = crc ^ r ^ TABLE[(byte & 0xf) as usize];

        let r = TABLE[(crc & 0xf) as usize];
        crc = (crc >> 4) & 0x0fff;
        crc = crc ^ r ^ TABLE[((byte >> 4) & 0xf) as usize];
    }
    crc
}

mod cal0 {
    use std::convert::TryInto;

    pub fn verify_block(data: &[u8]) -> bool {
        let len = data.len();
        let block = &data[..len - 2];
        let checksum = u16::from_le_bytes(data[len - 2..].try_into().unwrap());

        super::get_crc_16(block) == checksum    
    }
}
