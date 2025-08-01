use binread::BinRead;
use binwrite::BinWrite;
use dmw3_consts;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Debug;

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct EnemyStats {
    pub digimon_id: u16,

    pub droppable_item: u16,

    pub drop_rate: u16,

    pub some_index: u16,

    pub attack: u16,

    pub move_1: u16,

    pub move_2: u16,

    pub str: i16,

    pub def: i16,

    pub spt: i16,

    pub wis: i16,

    pub spd: i16,

    pub fir_res: i16,

    pub wtr_res: i16,

    pub ice_res: i16,

    pub wnd_res: i16,

    pub thd_res: i16,

    pub mch_res: i16,

    pub drk_res: i16,

    pub psn_rate: u16,

    pub par_rate: u16,

    pub cnf_rate: u16,

    pub slp_rate: u16,

    pub ko_rate: u16,

    digimon_type: u16,

    moveset_1: Moveset,

    moveset_2: Moveset,

    moveset_3: Moveset,

    moveset_4: Moveset,

    counter_moveset: Moveset,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, PartialEq, Eq, Hash)]
pub struct EncounterData {
    pub digimon_id: u32,

    pub lv: u16,

    pub max_hp: u16,

    pub max_mp: u16,

    pub multiplier: u16,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, PartialEq, Eq, Hash)]
pub struct PartyData {
    pub encounters: [Pointer; 3],

    pub ambush_rate: u8,

    pub p_type: u8,

    pub poison_immunity: u8,

    pub paralysis_immunity: u8,

    pub confuse_immunity: u8,

    pub sleep_immunity: u8,

    pub one_hit_ko_immunity: u8,

    pub drain_immunity: u8,

    pub unk_immunity: u8,

    pub steal_immunity: u8,

    pub power_down_immunity: u8,

    pub defense_down_immunity: u8,

    pub speed_down_immunity: u8,

    pub escape_immunity: u8,

    _padding: u16,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct PartyExpBits {
    pub dv_exp: u32,
    pub exp: u32,
    pub bits: u32,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
struct Moveset {
    action: u8,
    comparator: u8,
    value: u16,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct DigivolutionData {
    pub digimon_id: u16,

    pub str: u16,

    pub def: u16,

    pub spt: u16,

    pub wis: u16,

    pub spd: u16,

    pub chr: u16,

    pub fir_res: u16,

    pub wtr_res: u16,

    pub ice_res: u16,

    pub wnd_res: u16,

    pub thd_res: u16,

    pub mch_res: u16,

    pub drk_res: u16,

    pub attack: u16,

    #[br(count = 5)]
    pub tech: Vec<u16>,

    pub ori_tech: u16,

    pub dna_dv_tech: u16,

    pub psn_rate: u8,

    pub par_rate: u8,

    pub cnf_rate: u8,

    pub slp_rate: u8,

    pub ko_rate: u8,

    #[br(count = 5)]
    pub tech_learn_level: Vec<u8>,

    pub ori_tech_learn_level: u8,

    #[br(count = 5)]
    pub tech_load_level: Vec<u8>,

    dv_exp_modifier: u8,

    pub dna_dv_idx: u8,

    pub exp_modifier: u8,

    pub starting_hp: u8,

    pub starting_mp: u8,

    pub hp_modifier: u8,

    pub mp_modifier: u8,

    #[br(count = 6)]
    pub stat_offsets: Vec<u8>,

    #[br(count = 7)]
    pub res_offsets: Vec<u8>,

    #[br(count = 5)]
    pub blast_indices: Vec<u8>,

    pub dv_index: u8,

    #[br(count = 2)]
    unk_arr_1: Vec<u8>,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct Shop {
    pub item_count: u32,
    pub items: Pointer,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Hash, Eq, Ord, Default)]
pub struct Pointer {
    pub value: u32,
}

impl PartialEq for Pointer {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Pointer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct MoveData {
    pub mp: u16,
    pub power: u16,
    pub move_type: u8,
    unk1: u8,
    pub accuracy: u8,
    pub boosted_stat: u8,
    pub boost: u8,
    pub effective_against: u8,
    pub hit_effect: u8,
    pub effect_rate: u8,
    pub effect_value: u8,
    unk4: u32,
    pub freq: u8,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct ItemShopData {
    unk_ptr: Pointer,
    pub buy_price: u16,
    pub sell_price: u16,
    unk: u32,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct DigivolutionCondition {
    pub index: u32,      // +1
    pub dv_index_1: u16, // +1
    pub rq_level_1: u16,
    pub dv_index_2: u16, // +1
    pub rq_level_2: u16,
    pub rq_type: u16,
    pub rq: u16,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct DigivolutionConditions {
    #[br(count = dmw3_consts::DIGIVOLUTION_COUNT)]
    pub conditions: Vec<DigivolutionCondition>,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct StageLoadData {
    pub stage_id: u32,
    pub file_index: u32,
    entry_function: Pointer,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct Environmental {
    #[br(count = 2)]
    pub conditions: Vec<u32>,
    pub environmental_type: u16,
    pub next_stage_id: u16,
    pub next_stage_x: u16,
    pub next_stage_y: u16,
    next_stage_direction: u16,
    unk: u16,
    unk1: u32,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct EntityData {
    pub conditions: Pointer,
    pub logic: Pointer,
    pub sprite: u16,
    sprite_buffer_index: u16,
    pub x: u16,
    pub y: u16,
    pub direction: u16,
    padding: u16,
}

#[derive(BinRead, Debug, Copy, Clone, BinWrite)]
pub struct ScriptConditionStep {
    pub bitfield: u16,
    pub flag: u16,
}

impl ScriptConditionStep {
    pub fn is_last_step(self) -> bool {
        return self.bitfield == 0xffff && self.flag == 0;
    }
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct EntityLogic {
    pub conditions: Pointer,
    pub script: Pointer,
    pub text_index: u32,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct MapColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub tint: u8,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct StageEncounter {
    pub team_id: u32,
    pub stage: u32,
    pub music: u32,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct StageEncounterArea {
    pub steps_inddex: u32,
    pub teams: [Pointer; 8],
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct StageEncounters {
    pub unk1: i32,
    pub unk2: i32,
    pub areas: [Pointer; 5],
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct CardShopData {
    pub shop_id: i32,
    _card_count: i32,
    pub items: Pointer,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct CardPricing {
    pub card_id: i16,
    pub pricing: i16,
}

#[derive(BinRead, Debug, Clone, BinWrite)]
pub struct BoosterData {
    pub booster_item_id: i32,
    pub slots: [Pointer; 6],
}

impl Pointer {
    pub fn to_index(&self) -> u32 {
        self.value - 0x8000f800
    }

    pub fn to_index_overlay(&self, index: u32) -> u32 {
        self.value - index
    }

    pub fn from_instruction(buf: &[u8]) -> Pointer {
        let lui_index = buf
            .chunks(4)
            .rev()
            .position(|x| {
                x[2] == dmw3_consts::LUI_INSTRUCTION[0] && x[3] == dmw3_consts::LUI_INSTRUCTION[1]
            })
            .unwrap();

        let addiu_index = buf
            .chunks(4)
            .rev()
            .position(|x| x[3] == dmw3_consts::ADDIU)
            .unwrap();

        let lui_slice = &buf[buf.len() - (lui_index + 1) * 4..];
        let addiu_slice = &buf[buf.len() - (addiu_index + 1) * 4..];

        let bp = u16::from_le_bytes([lui_slice[0], lui_slice[1]]);
        let lp = i16::from_le_bytes([addiu_slice[0], addiu_slice[1]]);

        Pointer {
            value: (((bp as u32) << 16) as i32 + lp as i32) as u32,
        }
    }

    // pub fn from_index(index: u32) -> u32 {
    //     index + 0x8000f800
    // }

    pub fn from_index_overlay(index: u32, overlay: u32) -> Pointer {
        Pointer {
            value: index + overlay,
        }
    }

    pub fn is_valid(&self) -> bool {
        return 0x80000000 <= self.value && self.value <= 0x80100000;
    }

    pub fn null(&self) -> bool {
        return self.value == 0;
    }
}

impl From<&[u8]> for Pointer {
    fn from(buf: &[u8]) -> Self {
        Pointer {
            value: u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
        }
    }
}

impl From<[u8; 4]> for Pointer {
    fn from(buf: [u8; 4]) -> Self {
        Pointer {
            value: u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
        }
    }
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite)]
pub struct ScreenNameMapping {
    pub sector_name_idx: u8,
    pub screen_name_idx: u8,
    pub stage_id: u16,
}
