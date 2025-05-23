pub const CARDMON_MIN: u16 = 0x1c9;
pub const CARDMON_MAX: u16 = 0x1d0;
pub const BOSSES: [u16; 26] = [
    0x46, 0x7c, 0x8d, 0xb2, 0x151, 0x164, 0x166, 0x1b4, 0x1ba, 0x1bb, 0x1bc, 0x1bd, 0x1be, 0x1bf,
    0x1c0, 0x1c1, 0x1c2, 0x1c3, 0x1c4, 0x1c5, 0x1c6, 0x1c7, 0x1c8, 0x1d1, 0x1d2, 0x1d3,
];

pub const TRICERAMON_ID: u16 = 0xcb;
pub const ZANBAMON_ID: u16 = 0x151;
pub const GALACTICMON_1ST_PHASE: u16 = 0x1ba;
pub const GALACTICMON_IDS: [u16; 3] = [0x1ba, 0x1d2, 0x1d3];

pub const STATS_FILE: &str = "AAA/PRO/SDIGIEDT.PRO";
pub const ENCOUNTERS_FILE: &str = "AAA/PRO/FIELDSTG.PRO";
pub const SHOPS_FILE: &str = "AAA/PRO/STITSHOP.PRO";
pub const CARD_SHOPS_FILE: &str = "AAA/PRO/STCRDSHP.PRO";
pub const EXP_FILE: &str = "AAA/PRO/STFGTREP.PRO";
pub const MAP_FILE: &str = "AAA/PRO/FIELDSTG.PRO";
pub const PACK_SELECT_FILE: &str = "AAA/PRO/STPLNMET.PRO";

pub const PACKS: &[u8] = b"\x00\x06\x07\x02\x03\x06\x01\x05\x07";
pub const PARTY_DV_EXP: &[u8] = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0c\x00\x00\x00\x78\x00\x00\x00\xf0\x00\x00\x00";

pub const SHOPS_LEN: usize = 30;

pub const OVERLAY_ADDRESS: u32 = 0x800100c4;
pub const STAGE_ADDRESS: u32 = 0x800100c8;

pub const LI_INSTRUCTION: &[u8; 2] = b"\x02\x24";
pub const TALK_FILE_ADDIU: &[u8; 2] = b"\x42\x24";
pub const JR_RA_INSTRUCTION: &[u8; 4] = b"\x08\x00\xe0\x03";
pub const LUI_INSTRUCTION: &[u8; 2] = b"\x02\x3c";

pub const ENVIRONMENTAL_INSTRUCTION: &[u8; 4] = b"\x14\x00\x22\xae";
pub const ENTITIES_INSTRUCTION: &[u8; 4] = b"\x4c\x00\x22\xae";
pub const TALK_FILE_INSTRUCTION: &[u8; 4] = b"\x44\x00\x22\xae";
pub const STAGE_ENCOUNTERS_INSTRUCTION: &[u8; 4] = b"\x20\x00\x22\xae";
pub const STAGE_COLOR_INSTRUCTION_HALF: &[u8; 2] = b"\x38\x00";

pub const ADDIU: u8 = 0x24;

// min is 0, max is how much space there is for shops
pub const MIN_SHOP_ITEMS: i64 = 0;
pub const MAX_SHOP_ITEMS: i64 = 37;

pub const MIN_STAT_RANGE: i64 = 0;
pub const MAX_STAT_RANGE: i64 = 150;

// don't want free items
pub const MIN_SELL_PRICE: i64 = 1;
// max u16 / 2
pub const MAX_SELL_PRICE: i64 = 32768;

pub const TNT_BALL_ID: u16 = 0x5a;

pub const MULTI_HIT: u8 = 0x9;

pub const CHAMPIONS: [u16; 10] = [
    0x182, 0x103, 0x183, 0x176, 0x14, 0x16f, 0x184, 0x5, 0x104, 0xea,
];

pub const ULTIMATES: [u16; 13] = [
    0x186, 0x187, 0x185, 0xd3, 0xc, 0x1b, 0x177, 0x13, 0x38, 0x170, 0xfe, 0x1a, 0x6,
];

pub const MEGAS: [u16; 13] = [
    0x188, 0x189, 0x18a, 0xd5, 0x94, 0x171, 0x178, 0xd6, 0xc4, 0x90, 0x10b, 0x3b, 0x42,
];

pub const MEGAPLUS: [u16; 5] = [0x96, 0xe6, 0x174, 0x17a, 0x167];

pub const ULTRAS: [u16; 3] = [0x97, 0x179, 0x17d];

pub const ROOKIE_COUNT: usize = 8;
pub const DIGIVOLUTION_COUNT: usize = 44;

pub const MIN_STAT_AFFINITY: u8 = 1;
pub const MAX_STAT_AFFINITY: u8 = 5;

pub const STAGE_LOAD_DATA_LENGTH: usize = 241;

pub const ITEM_BOX_SPRITES: &[u16] = &[
    0x21, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x58, 0x154, 0x155, 0x156,
    0x157, 0x158, 0x15b,
];

pub const ITEM_NAMES: &str = "SITMNAM.BIN";

pub const SHOP_NAMES_MAPPED: [u32; 30] = [
    0x11, 0x01, 0x05, 0x12, // 0-3
    0x06, 0x19, 0x0D, 0x18, // 4-7
    0x04, 0x0C, 0x1D, 0x1E, // 8-11
    0x10, 0x15, 0x09, 0x13, // 12-15
    0x02, 0x07, 0x14, 0x08, // 16-19
    0x1A, 0x0E, 0x17, 0x03, // 20-23
    0x0B, 0x1B, 0x1C, 0x0F, // 24-27
    0x16, 0xA,
];

pub const SCREEN_NAME_MAPPING: &[u8] = b"\x0b\x01\x00\x02";

pub const CHARISMA_VALUES: [u32; 15] = [
    60,   // 0x003C
    150,  // 0x0096
    210,  // 0x00D2
    285,  // 0x011D
    378,  // 0x017A
    492,  // 0x01EC
    630,  // 0x0276
    795,  // 0x031B
    990,  // 0x03DE
    1218, // 0x04C2
    1482, // 0x05CA
    1785, // 0x06F9
    2049, // 0x0801
    2277, // 0x08E5
    2472, // 0x09A8
];
