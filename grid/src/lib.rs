pub struct GridInfo {
    pub width: u8,
    pub height: u8,
    pub blocks_128_indices: Vec<u8>,
}

pub struct Grid {
    pub info: GridInfo,
    pub blocks_64_indices: Vec<[[u8; 2]; 2]>,
    pub blocks_32_indices: Vec<[[u16; 2]; 2]>,
    pub blocks_16_indices: Vec<[[u16; 2]; 2]>,
    pub blocks_8_indices: Vec<[[u16; 2]; 2]>,
    pub blocks_8: Vec<[[u8; 8]; 8]>,
}

pub fn get_grid_value(grid_s: &Grid, x: u32, y: u32) -> u8 {
    let block_128_idx = grid_s.info.blocks_128_indices
        [((y >> 7) * (grid_s.info.width as u32)) as usize + (x >> 7) as usize]
        as usize;

    let block_128_q_i = ((x & 64) > 0) as usize;
    let block_128_q_j = ((y & 64) > 0) as usize;

    let block_64_idx =
        grid_s.blocks_64_indices[block_128_idx][block_128_q_j][block_128_q_i] as usize;

    let block_64_q_i = ((x & 32) > 0) as usize;
    let block_64_q_j = ((y & 32) > 0) as usize;

    let block_32_idx = grid_s.blocks_32_indices[block_64_idx][block_64_q_j][block_64_q_i] as usize;

    let block_32_q_i = ((x & 16) > 0) as usize;
    let block_32_q_j = ((y & 16) > 0) as usize;

    let block_16_idx = grid_s.blocks_16_indices[block_32_idx][block_32_q_j][block_32_q_i] as usize;

    let block_16_q_i = ((x & 8) > 0) as usize;
    let block_16_q_j = ((y & 8) > 0) as usize;

    let block_8_idx = grid_s.blocks_8_indices[block_16_idx][block_16_q_j][block_16_q_i] as usize;

    let block_8 = &grid_s.blocks_8[block_8_idx];

    let i = (x & 7) as usize;
    let j = (y & 7) as usize;

    block_8[j][i]
}
