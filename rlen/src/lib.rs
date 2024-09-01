use std::iter;

pub fn rlen_decode(bytes: &[u8]) -> Result<Vec<u8>, String> {
    if bytes[0..4] != *b"RLEN" {
        return Err("File not run length encoded".into());
    }

    let mut result = Vec::new();

    let final_length = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;

    let mut i = 8;
    let mut cur_pos = 0;

    while cur_pos < final_length {
        let blen = bytes[i] as usize;

        if blen < 0x80 {
            for j in i + 1..i + 1 + blen {
                result.push(bytes[j]);
            }

            i += blen + 1;
            cur_pos += blen;
        } else {
            let byte = bytes[i + 1];

            result.extend(iter::repeat(byte).take(blen - 0x80));

            i += 2;
            cur_pos += blen - 0x80;
        }
    }

    if result.len() != final_length {
        return Err("File length mismatch".into());
    }

    Ok(result)
}

pub fn rlen_encode(bytes: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = "RLEN".into();

    result.extend((bytes.len() as u32).to_le_bytes().iter());

    let mut i = 0;

    while i < bytes.len() {
        if i >= bytes.len() - 1 {
            result.push(0x1);
            result.push(bytes[i]);

            break;
        }

        if bytes[i] != bytes[i + 1] {
            let mut j = i;

            while j < bytes.len() - 1 && bytes[j] != bytes[j + 1] && j - i < 127 {
                j += 1;
            }

            result.push((j - i) as u8);
            result.extend_from_slice(&bytes[i..j]);

            i = j;
        } else {
            let mut j = i;

            while j < bytes.len() - 1 && bytes[j] == bytes[j + 1] && j - i < 126 {
                j += 1;
            }

            j += 1;

            result.push((j - i) as u8 + 0x80);
            result.push(bytes[i]);

            i = j;
        }
    }

    result
}
