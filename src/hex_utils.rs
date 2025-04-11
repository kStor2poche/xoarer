use anyhow::{Result, anyhow};

pub fn hex_decode(s: String) -> Result<Vec<u8>> {
    let s = if let Some("0x") = s.get(0..2) {
        s.get(2..).expect("empty s (only got 0x prefix)")
    } else {
        s.as_str()
    };
    Ok(hex::decode(s)?)
}

pub fn hex_bytes_to_usize(mut bytes: Vec<u8>) -> Result<usize> {
    if bytes.len() > 8 {
        return Err(anyhow!("no can do len > 8 bytes"));
    }
    while bytes.len() != 8 {
        bytes.insert(0, 0);
    }
    Ok(usize::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}
