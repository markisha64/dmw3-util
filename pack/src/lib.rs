use binrw::{BinWrite, BinReaderExt};
use std::io::Cursor;

#[derive(Clone)]
pub struct Packed {
    pub files: Vec<Vec<u8>>,
}

impl Packed {
    pub fn file_size(&self) -> usize {
        let header_length = self.files.len() * 4;
        let files_length = self.files.iter().fold(0, |pv, cv| pv + cv.len());

        header_length + files_length
    }

    pub fn file_size_text(&self) -> usize {
        let header_length = self.files.len() * 4 + 4;
        let files_length = self.files.iter().fold(0, |pv, cv| pv + cv.len());

        header_length + files_length
    }

    pub fn iter(&self) -> std::ops::Range<usize> {
        0..self.files.len()
    }

    pub fn to_bytes_text(&self) -> Vec<u8> {
        let mut result = Cursor::new(Vec::new());
        let mut i: u32 = (self.files.len() * 4) as u32 + 4;

        (self.files.len() as u32).write_le(&mut result).unwrap();

        for file in &self.files {
            i.write_le(&mut result).unwrap();
            i += file.len() as u32;
        }

        for file in &self.files {
            result.get_mut().extend(file);
        }

        result.into_inner()
    }
}

impl Packed {
    pub fn from_text(file: Vec<u8>) -> Packed {
        let mut reader = Cursor::new(&file);
        let mut files: Vec<Vec<u8>> = Vec::new();

        let length: u32 = reader.read_le().unwrap();

        if length == 0 {
            return Packed { files };
        }

        let mut offsets: Vec<u32> = Vec::new();
        for _ in 0..length {
            offsets.push(reader.read_le().unwrap());
        }

        for i in 0..offsets.len() - 1 {
            if offsets[i] >= offsets[i + 1] {
                files.push(Vec::new());
                continue;
            }

            files.push(file[offsets[i] as usize..offsets[i + 1] as usize].into());
        }

        files.push(file[*offsets.last().unwrap() as usize..].into());
        Packed { files }
    }
}

impl From<Vec<u8>> for Packed {
    fn from(file: Vec<u8>) -> Self {
        let mut reader = Cursor::new(&file);
        let mut files: Vec<Vec<u8>> = Vec::new();

        let first_offset: u32 = reader.read_le().unwrap();

        if first_offset == 0 {
            return Packed { files };
        }

        let mut offsets: Vec<u32> = vec![first_offset];
        for _ in 1..first_offset / 4 {
            offsets.push(reader.read_le().unwrap());
        }

        let len = file.len() as u32;

        // currently omitting overlaps, do better handling eventually
        let mut last_offset = offsets[0];
        for i in 0..offsets.len() - 1 {
            if offsets[i] < last_offset {
                files.push(Vec::new());
                continue;
            }

            let sidx = offsets[i + 1..]
                .iter()
                .find(|x| **x > offsets[i])
                .unwrap_or(&len);

            last_offset = *sidx;

            files.push(file[offsets[i] as usize..*sidx as usize].into());
        }

        if *offsets.last().unwrap() == 0 {
            files.push(Vec::new());
        } else {
            files.push(file[*offsets.last().unwrap() as usize..].into());
        }

        Packed { files }
    }
}

impl From<Packed> for Vec<u8> {
    fn from(val: Packed) -> Self {
        let mut result = Cursor::new(Vec::new());
        let mut i: u32 = (val.files.len() * 4) as u32;

        for file in &val.files {
            if file.len() == 0 {
                (0u32).write_le(&mut result).unwrap();
            } else {
                i.write_le(&mut result).unwrap();
            }

            i += file.len() as u32;
        }

        for file in val.files {
            result.get_mut().extend(file);
        }

        result.into_inner()
    }
}
