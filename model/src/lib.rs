#[derive(BinRead)]
struct PartPacked {
    parent_index: u32,
    geometry: u32,
    animation: u32,
}

#[derive(BinRead)]
struct AnimationInst {
    t: u16,
    len: u16,
    c: u16,
    d: u16,
}

#[derive(BinRead, Debug, Clone)]
struct AnimationFrame {
    vx: i16,
    vy: i16,
    vz: i16,
    id: u16,
}

#[derive(Debug, Clone)]
struct AnimationFrames {
    translation: AnimationFrame,
    rotation: AnimationFrame,
    scale: AnimationFrame,
}

#[derive(Clone, Debug)]
struct Part {
    vert_offset: usize,
    vert_len: usize,
    tex_offset: usize,
    tex_len: usize,
    _animation: usize,
}

#[derive(BinRead)]
struct Header {
    texture_offset: u32,
    _part_count: u32,
    #[br(count=_part_count)]
    parts: Vec<PartPacked>,
}
