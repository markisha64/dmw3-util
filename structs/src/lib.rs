use binrw::{BinRead, BinWrite};
use dmw3_consts;
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Debug;

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
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

#[derive(BinRead, Debug, Clone, Copy, BinWrite, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[brw(little)]
pub struct EncounterData {
    pub digimon_id: u32,

    pub lv: u16,

    pub max_hp: u16,

    pub max_mp: u16,

    pub multiplier: u16,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[brw(little)]
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

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct PartyExpBits {
    pub dv_exp: u32,
    pub exp: u32,
    pub bits: u32,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
struct Moveset {
    action: u8,
    comparator: u8,
    value: u16,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
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

    pub tech: [u16; 5],

    pub ori_tech: u16,

    pub dna_dv_tech: u16,

    pub psn_rate: u8,

    pub par_rate: u8,

    pub cnf_rate: u8,

    pub slp_rate: u8,

    pub ko_rate: u8,

    pub tech_learn_level: [u8; 5],

    pub ori_tech_learn_level: u8,

    pub tech_load_level: [u8; 5],

    dv_exp_modifier: u8,

    pub dna_dv_idx: u8,

    pub exp_modifier: u8,

    pub starting_hp: u8,

    pub starting_mp: u8,

    pub hp_modifier: u8,

    pub mp_modifier: u8,

    pub stat_offsets: [u8; 6],

    pub res_offsets: [u8; 7],

    pub blast_indices: [u8; 5],

    pub dv_index: u8,

    unk: u16,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Shop {
    pub item_count: u32,
    pub items: Pointer,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Hash, Eq, Ord, Default)]
#[brw(little)]
pub struct Pointer {
    pub value: u32,
}

impl Serialize for Pointer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{:08X}", self.value))
    }
}

impl<'de> Deserialize<'de> for Pointer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let s = s
            .strip_prefix("0x")
            .or_else(|| s.strip_prefix("0X"))
            .ok_or_else(|| {
                serde::de::Error::custom("expected pointer string starting with '0x'")
            })?;

        if s.len() != 8 {
            return Err(serde::de::Error::custom(
                "expected exactly 8 hexadecimal digits",
            ));
        }

        let value = u32::from_str_radix(s, 16).map_err(serde::de::Error::custom)?;

        Ok(Pointer { value })
    }
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

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
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

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ItemShopData {
    unk_ptr: Pointer,
    pub buy_price: u16,
    pub sell_price: u16,
    unk: u32,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct DigivolutionCondition {
    pub index: u32,      // +1
    pub dv_index_1: u16, // +1
    pub rq_level_1: u16,
    pub dv_index_2: u16, // +1
    pub rq_level_2: u16,
    pub rq_type: u16,
    pub rq: u16,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct DigivolutionConditions {
    #[br(count = dmw3_consts::DIGIVOLUTION_COUNT)]
    pub conditions: Vec<DigivolutionCondition>,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct StageLoadData {
    pub stage_id: u32,
    pub file_index: u32,
    entry_function: Pointer,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct Environmental {
    pub conditions: [ScriptConditionStep; 2],
    pub environmental_type: u16,
    pub next_stage_id: u16,
    pub next_stage_x: u16,
    pub next_stage_y: u16,
    next_stage_direction: u16,
    unk: u16,
    unk1: u32,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
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

/// The 7-bit type field from a `ScriptConditionStep` bitfield.
///
/// The raw bitfield is a `u16` packed as:
///   - bits  [8:0]  → `value` (9 bits)
///   - bits [15:9]  → type (7 bits, stored as `condition_type`)
///
/// The discriminant of each variant matches the value obtained via
/// `bitfield >> 8 & 0xfe` in the original code (i.e. the 7-bit type
/// shifted left by one, with bit 8 of the value masked away).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptConditionType {
    /// c_type 0  – "Tamer" flag
    TamerFlag,
    /// c_type 2  – Item-box opened flag
    ItemBox,
    /// c_type 4  – Auction done flag
    Auction,
    /// c_type 6  – "DDNA Megas" flag
    DdnaMegas,
    /// c_type 8  – Unknown flag group 2
    Unk2,
    /// c_type 10 – "Bosses" flag
    Bosses,
    /// c_type 12 – "A.o.A." flag
    AoA,
    /// c_type 14 – "Battled Tamer" flag
    BattledTamer,
    /// c_type 16 – Unknown flag group 6
    Unk6,
    /// c_type 24 – Unknown flag group 7
    Unk7,
    /// c_type 26 – "NPC I" flag
    NpcI,
    /// c_type 28 – "NPC II" flag
    NpcII,
    /// c_type 32 – Area visited flag
    AreaVisited,
    /// c_type 64 – Story flag
    Story,
    /// c_type 96 – Quest condition
    Quest,
    /// c_type 112 – Complex condition (references a `ComplexScriptConditionStep`)
    Complex,
    /// c_type 114 – Total charisma check
    Charisma,
    /// c_type 116 – Start scripted battle (script only)
    ScriptedBattle,
    /// c_type 118 – Start card battle (script only)
    CardBattle,
    /// c_type 120 – Start stronger card battle (script only)
    StrongerCardBattle,
    /// c_type 122 – Inn or shop (script only)
    InnOrShop,
    /// c_type 128..=142 (even) – Item ownership / give/take item; holds the raw `c_type` byte.
    Item(u8),
    /// c_type 144 – Start cutscene (script only)
    Cutscene,
    /// Any unrecognised type; holds the raw `c_type` byte (`bitfield >> 8 & 0xfe`).
    Unknown(u8),
}

impl ScriptConditionType {
    /// Returns the `c_type` value (`bitfield >> 8 & 0xfe`) for this variant.
    pub fn to_raw(self) -> u8 {
        match self {
            ScriptConditionType::TamerFlag => 0,
            ScriptConditionType::ItemBox => 2,
            ScriptConditionType::Auction => 4,
            ScriptConditionType::DdnaMegas => 6,
            ScriptConditionType::Unk2 => 8,
            ScriptConditionType::Bosses => 10,
            ScriptConditionType::AoA => 12,
            ScriptConditionType::BattledTamer => 14,
            ScriptConditionType::Unk6 => 16,
            ScriptConditionType::Unk7 => 24,
            ScriptConditionType::NpcI => 26,
            ScriptConditionType::NpcII => 28,
            ScriptConditionType::AreaVisited => 32,
            ScriptConditionType::Story => 64,
            ScriptConditionType::Quest => 96,
            ScriptConditionType::Complex => 112,
            ScriptConditionType::Charisma => 114,
            ScriptConditionType::ScriptedBattle => 116,
            ScriptConditionType::CardBattle => 118,
            ScriptConditionType::StrongerCardBattle => 120,
            ScriptConditionType::InnOrShop => 122,
            ScriptConditionType::Item(v) => v,
            ScriptConditionType::Cutscene => 144,
            ScriptConditionType::Unknown(v) => v,
        }
    }

    /// Construct from the raw `c_type` value (`bitfield >> 8 & 0xfe`).
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => ScriptConditionType::TamerFlag,
            2 => ScriptConditionType::ItemBox,
            4 => ScriptConditionType::Auction,
            6 => ScriptConditionType::DdnaMegas,
            8 => ScriptConditionType::Unk2,
            10 => ScriptConditionType::Bosses,
            12 => ScriptConditionType::AoA,
            14 => ScriptConditionType::BattledTamer,
            16 => ScriptConditionType::Unk6,
            24 => ScriptConditionType::Unk7,
            26 => ScriptConditionType::NpcI,
            28 => ScriptConditionType::NpcII,
            32 => ScriptConditionType::AreaVisited,
            64 => ScriptConditionType::Story,
            96 => ScriptConditionType::Quest,
            112 => ScriptConditionType::Complex,
            114 => ScriptConditionType::Charisma,
            116 => ScriptConditionType::ScriptedBattle,
            118 => ScriptConditionType::CardBattle,
            120 => ScriptConditionType::StrongerCardBattle,
            122 => ScriptConditionType::InnOrShop,
            128..=142 => ScriptConditionType::Item(raw),
            144 => ScriptConditionType::Cutscene,
            other => ScriptConditionType::Unknown(other),
        }
    }
}

/// A single step in a script condition / script action list.
///
/// On disk this is two `u16` values in little-endian order.  The first `u16`
/// is a packed bitfield:
///
/// ```text
/// bit  15 14 13 12 11 10  9 | 8  7  6  5  4  3  2  1  0
///      ←── condition_type ──  ←───────── value ──────────
///                  (7 bits)               (9 bits)
/// ```
///
/// The list is terminated by a sentinel entry where `value == 0x1ff`,
/// `condition_type == Unknown(0xfe)` (i.e. the packed u16 == `0xffff`) and
/// `flag == 0`.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ScriptConditionStep {
    EndStep,
    Step {
        /// 9-bit payload (bits [8:0] of the packed `u16`).
        value: u16,
        /// 7-bit type field (bits [15:9] of the packed `u16`).
        condition_type: ScriptConditionType,
        flag: u16,
    },
}

impl ScriptConditionStep {
    pub fn is_last_step(&self) -> bool {
        matches!(self, ScriptConditionStep::EndStep)
    }
}

impl binrw::BinRead for ScriptConditionStep {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let raw_bitfield = u16::read_options(reader, endian, ())?;
        let flag = u16::read_options(reader, endian, ())?;

        let step = match raw_bitfield == 0xffff && flag == 0x0000 {
            true => ScriptConditionStep::EndStep,
            false => {
                // value: bits [8:0]
                let value = raw_bitfield & 0x1ff;
                // c_type: bits [15:9] shifted into [7:1], matching `>> 8 & 0xfe`
                let c_type = ((raw_bitfield >> 8) & 0xfe) as u8;
                let condition_type = ScriptConditionType::from_raw(c_type);

                ScriptConditionStep::Step {
                    value,
                    condition_type,
                    flag,
                }
            }
        };

        Ok(step)
    }
}

impl binrw::BinWrite for ScriptConditionStep {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        match self {
            Self::EndStep => {
                0x0000ffffu32.write_options(writer, endian, ())?;
            }
            Self::Step {
                value,
                condition_type,
                flag,
            } => {
                // Re-pack: type occupies bits [15:9], value occupies bits [8:0].
                // c_type = raw >> 8 & 0xfe  →  raw bits [15:9] = c_type >> 1
                let c_type = condition_type.to_raw() as u16;
                let raw_bitfield = (value & 0x1ff) | ((c_type << 8) & 0xfe00);
                raw_bitfield.write_options(writer, endian, ())?;
                flag.write_options(writer, endian, ())?;
            }
        }

        Ok(())
    }
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ComplexScriptConditionStep {
    pub id: u8,
    pub operation_and_type: u8,
    pub value: u8,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct EntityLogic {
    pub conditions: Pointer,
    pub script: Pointer,
    pub text_index: u32,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct MapColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub tint: u8,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct StageEncounter {
    pub team_id: u32,
    pub stage: u32,
    pub music: u32,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct StageEncounterArea {
    pub steps_inddex: u32,
    pub teams: [Pointer; 8],
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct StageEncounters {
    pub unk1: i32,
    pub unk2: i32,
    pub areas: [Pointer; 5],
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct CardShopData {
    pub shop_id: i32,
    _card_count: i32,
    pub items: Pointer,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct CardPricing {
    pub card_id: i16,
    pub pricing: i16,
}

#[derive(BinRead, Debug, Clone, BinWrite, Serialize, Deserialize)]
#[brw(little)]
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

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct ScreenNameMapping {
    pub sector_name_idx: u8,
    pub screen_name_idx: u8,
    pub stage_id: u16,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct QuestRange {
    pub min: u8,
    pub max: u8,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct StageOverride {
    pub var1: i16,
    pub var2: i16,
    pub overrides: Pointer,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct EnvironmentalOverride {
    pub next_stage_id: u16,
    pub var1: i16,
    pub var2: i16,
    pub next_stage_x: i16,
    pub next_stage_y: i16,
    pub next_stage_direction: u16,
    pub next: Pointer,
}

#[derive(BinRead, Debug, Clone, Copy, BinWrite, Serialize, Deserialize)]
#[brw(little)]
pub struct MaskObject {
    pub show: u8,
    pub instance_id: u8,
    pub z: u8,
    pub type1: u8,
    pub id: u8,
    pub type2: u8,
    pub min_frame: u8,
    pub max_frame: u8,
    pub animation_speed: u8,
    pub current_frame: u8,

    pub x: u16,
    pub y: u16,

    pub unk1: i16,
    pub frame_counter: i16,
}
