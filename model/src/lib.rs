use binread::{io::Cursor, BinRead};

#[derive(BinRead)]
pub struct PartPacked {
    pub parent_index: u32,
    pub geometry: u32,
    pub animation: u32,
}

#[derive(BinRead)]
pub struct AnimationInst {
    pub t: u16,
    pub len: u16,
    pub c: u16,
    pub d: u16,
}

#[derive(BinRead, Debug, Clone)]
pub struct AnimationFrame {
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
    pub id: u16,
}

#[derive(Debug, Clone)]
pub struct AnimationFrames {
    pub translation: AnimationFrame,
    pub rotation: AnimationFrame,
    pub scale: AnimationFrame,
}

#[derive(Clone, Debug)]
pub struct Part {
    pub vert_offset: usize,
    pub vert_len: usize,
    pub tex_offset: usize,
    pub tex_len: usize,
    pub _animation: usize,
}

#[derive(BinRead)]
pub struct Header {
    pub texture_offset: u32,
    pub _part_count: u32,
    #[br(count=_part_count)]
    pub parts: Vec<PartPacked>,
}
