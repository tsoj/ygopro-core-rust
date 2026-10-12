//! The state of a duel (ygopro-core's `duel`, `field` and `processor`, merged into one struct).

use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;

use crate::card::{Card, CardState};
use crate::carddb::CardDb;
use crate::constants::*;
use crate::effect::Effect;
use crate::types::*;
use crate::units::Unit;

/// `RNG::Xoshiro256StarStar`
#[derive(Clone, Debug)]
pub struct Xoshiro256 {
    s: [u64; 4],
}

impl Xoshiro256 {
    pub fn new(s: [u64; 4]) -> Self {
        Xoshiro256 { s }
    }
    #[inline]
    pub fn next(&mut self) -> u64 {
        let s = &mut self.s;
        let result = (s[1].wrapping_mul(5)).rotate_left(7).wrapping_mul(9);
        let t = s[1] << 17;
        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = s[3].rotate_left(45);
        result
    }
}

/// A message to the players (`duel::duel_message`).
#[derive(Clone, Debug, Default)]
pub struct Message {
    pub data: Vec<u8>,
}

impl Message {
    #[inline]
    pub fn u8(&mut self, v: impl IntoU64) -> &mut Self {
        self.data.push(v.into_u64() as u8);
        self
    }
    #[inline]
    pub fn u16(&mut self, v: impl IntoU64) -> &mut Self {
        self.data.extend_from_slice(&(v.into_u64() as u16).to_le_bytes());
        self
    }
    #[inline]
    pub fn u32(&mut self, v: impl IntoU64) -> &mut Self {
        self.data.extend_from_slice(&(v.into_u64() as u32).to_le_bytes());
        self
    }
    #[inline]
    pub fn u64(&mut self, v: impl IntoU64) -> &mut Self {
        self.data.extend_from_slice(&v.into_u64().to_le_bytes());
        self
    }
    #[inline]
    pub fn i32(&mut self, v: i32) -> &mut Self {
        self.data.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        self.data.extend_from_slice(b);
        self
    }
    /// `duel_message::write(loc_info)`
    pub fn loc(&mut self, l: LocInfo) -> &mut Self {
        self.u8(l.controler).u8(l.location).u32(l.sequence).u32(l.position)
    }
}

/// Integers written to messages: C++ `write<T>(value)` casts any integer (wrapping).
pub trait IntoU64 {
    fn into_u64(self) -> u64;
}
macro_rules! into_u64 {
    ($($t:ty),*) => { $(impl IntoU64 for $t { #[inline] fn into_u64(self) -> u64 { self as u64 } })* };
}
into_u64!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
impl IntoU64 for bool {
    #[inline]
    fn into_u64(self) -> u64 {
        self as u64
    }
}

/// `ProgressiveBuffer`: the response of a player, and values returned between units.
#[derive(Clone, Debug, Default)]
pub struct ProgressiveBuffer {
    pub data: Vec<u8>,
}

macro_rules! pb_access {
    ($at:ident, $set:ident, $t:ty) => {
        pub fn $at(&self, pos: usize) -> $t {
            const N: usize = std::mem::size_of::<$t>();
            if self.data.len() < (pos + 1) * N {
                return 0 as $t;
            }
            <$t>::from_le_bytes(self.data[pos * N..pos * N + N].try_into().unwrap())
        }
        pub fn $set(&mut self, pos: usize, val: $t) {
            const N: usize = std::mem::size_of::<$t>();
            if self.data.len() < (pos + 1) * N {
                self.data.resize((pos + 1) * N, 0);
            }
            self.data[pos * N..pos * N + N].copy_from_slice(&val.to_le_bytes());
        }
    };
}

impl ProgressiveBuffer {
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pb_access!(at_u8, set_u8, u8);
    pb_access!(at_i8, set_i8, i8);
    pb_access!(at_u16, set_u16, u16);
    pb_access!(at_i16, set_i16, i16);
    pb_access!(at_u32, set_u32, u32);
    pb_access!(at_i32, set_i32, i32);
    pb_access!(at_u64, set_u64, u64);
    pb_access!(at_i64, set_i64, i64);
    pub fn bit_get(&self, pos: usize) -> bool {
        let real_pos = pos / 8;
        if self.data.len() < real_pos + 1 {
            return false;
        }
        self.data[real_pos] & (1 << (pos % 8)) != 0
    }
    pub fn bit_toggle(&mut self, pos: usize, set: bool) {
        let real_pos = pos / 8;
        if self.data.len() < real_pos + 1 {
            self.data.resize(real_pos + 1, 0);
        }
        if set {
            self.data[real_pos] |= 1 << (pos % 8);
        } else {
            self.data[real_pos] &= !(1 << (pos % 8));
        }
    }
}

/// `tevent`
#[derive(Clone, Debug, Default)]
pub struct Event {
    pub trigger_card: Option<CardId>,
    pub event_cards: Option<Group>,
    pub reason_effect: Option<EffectId>,
    pub event_code: u32,
    pub event_value: u32,
    pub reason: u32,
    pub event_player: u8,
    pub reason_player: u8,
    pub global_id: u32,
}

/// `optarget`
#[derive(Clone, Debug, Default)]
pub struct OpTarget {
    pub op_cards: Option<Group>,
    pub op_count: u8,
    pub op_player: u8,
    pub op_param: i32,
}

/// `chain`
#[derive(Clone, Debug, Default)]
pub struct Chain {
    pub triggering_state: CardState,
    pub evt: Event,
    pub chain_count: u8,
    pub triggering_player: u8,
    pub triggering_controler: u8,
    pub triggering_position: u8,
    pub target_player: u8,
    pub disable_player: u8,
    pub triggering_summon_location: u8,
    pub triggering_summon_proc_complete: bool,
    pub was_just_sent: bool,
    pub chain_id: u16,
    pub triggering_location: u16,
    pub triggering_sequence: u32,
    pub triggering_status: u32,
    pub triggering_summon_type: u32,
    pub replace_op: Option<LuaFn>,
    pub target_param: i32,
    pub flag: u32,
    pub event_id: u32,
    pub triggering_effect: Option<EffectId>,
    pub target_cards: Option<Group>,
    pub disable_reason: Option<EffectId>,
    pub applied_chain_counters: Option<Vec<u32>>,
    /// `opmap` (an unordered_map by category): kept sorted by category
    pub opinfos: Vec<(u64, OpTarget)>,
    pub possibleopinfos: Vec<(u64, OpTarget)>,
}

/// `player_info`
#[derive(Clone, Debug, Default)]
pub struct PlayerInfo {
    pub lp: i32,
    pub start_lp: i32,
    pub start_count: i32,
    pub draw_count: i32,
    pub used_location: u32,
    pub disabled_location: u32,
    pub extra_p_count: u32,
    pub exchanges: u32,
    pub tag_index: u32,
    pub recharge: bool,
    pub list_mzone: Vec<Option<CardId>>,
    pub list_szone: Vec<Option<CardId>>,
    pub list_main: Vec<CardId>,
    pub list_grave: Vec<CardId>,
    pub list_hand: Vec<CardId>,
    pub list_remove: Vec<CardId>,
    pub list_extra: Vec<CardId>,
    pub extra_lists_main: Vec<Vec<CardId>>,
    pub extra_lists_hand: Vec<Vec<CardId>>,
    pub extra_lists_extra: Vec<Vec<CardId>>,
    pub extra_extra_p_count: Vec<u32>,
}

impl PlayerInfo {
    pub fn new(team: &PlayerOptions) -> Self {
        PlayerInfo {
            lp: team.starting_lp as i32,
            start_lp: team.starting_lp as i32,
            start_count: team.starting_draw_count as i32,
            draw_count: team.draw_count_per_turn as i32,
            list_mzone: vec![None; 7],
            list_szone: vec![None; 8],
            ..Default::default()
        }
    }
}

/// A `std::multimap<uint32_t, effect*>`: sorted by code, equal codes in insertion order.
#[derive(Clone, Debug, Default)]
pub struct EffectContainer {
    pub items: Vec<(u32, EffectId)>,
    /// A filter of the codes present: one bit per code hash (a code whose bit is clear has no effect
    /// here; most lookups are for absent codes)
    codes: u64,
}

#[inline]
fn code_bit(code: u32) -> u64 {
    1u64 << (code.wrapping_mul(0x9E37_79B9) >> 26)
}

impl EffectContainer {
    pub fn insert(&mut self, code: u32, e: EffectId) {
        let pos = self.items.partition_point(|&(c, _)| c <= code);
        self.items.insert(pos, (code, e));
        self.codes |= code_bit(code);
    }
    fn refilter(&mut self) {
        self.codes = self.items.iter().fold(0, |m, &(c, _)| m | code_bit(c));
    }
    /// Removes `e` (filed under `code`); returns whether it was there.
    pub fn remove(&mut self, code: u32, e: EffectId) -> bool {
        let lo = self.items.partition_point(|&(c, _)| c < code);
        let hi = self.items.partition_point(|&(c, _)| c <= code);
        if let Some(i) = self.items[lo..hi].iter().position(|&(_, x)| x == e) {
            self.items.remove(lo + i);
            self.refilter();
            return true;
        }
        false
    }
    pub fn remove_any(&mut self, e: EffectId) -> bool {
        if let Some(i) = self.items.iter().position(|&(_, x)| x == e) {
            self.items.remove(i);
            self.refilter();
            return true;
        }
        false
    }
    /// The effects filed under `code` (`equal_range`), in order.
    #[inline]
    pub fn range(&self, code: u32) -> Vec<EffectId> {
        if self.codes & code_bit(code) == 0 {
            return Vec::new();
        }
        self.range_slow(code)
    }
    fn range_slow(&self, code: u32) -> Vec<EffectId> {
        // (small containers: a scan beats two binary searches)
        if self.items.len() <= 16 {
            return self.items.iter().filter(|&&(c, _)| c == code).map(|&(_, e)| e).collect();
        }
        let lo = self.items.partition_point(|&(c, _)| c < code);
        let hi = self.items.partition_point(|&(c, _)| c <= code);
        self.items[lo..hi].iter().map(|&(_, e)| e).collect()
    }
    /// Whether an effect is filed under `code`.
    #[inline]
    pub fn has(&self, code: u32) -> bool {
        self.codes & code_bit(code) != 0 && self.items.iter().any(|&(c, _)| c == code)
    }
    pub fn all(&self) -> Vec<EffectId> {
        self.items.iter().map(|&(_, e)| e).collect()
    }
    pub fn contains(&self, e: EffectId) -> bool {
        self.items.iter().any(|&(_, x)| x == e)
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// `field_effect::grant_effect_container`: granting effect -> (card -> granted effect).
#[derive(Clone, Debug, Default)]
pub struct GrantEffects {
    /// sorted by the granting effect's initial id (the `sorted` view; `unsorted` is the same data)
    pub entries: Vec<(EffectId, Vec<(CardId, EffectId)>)>,
}

/// `field_effect`
#[derive(Clone, Debug, Default)]
pub struct FieldEffect {
    pub aura_effect: EffectContainer,
    pub ignition_effect: EffectContainer,
    pub activate_effect: EffectContainer,
    pub trigger_o_effect: EffectContainer,
    pub trigger_f_effect: EffectContainer,
    pub quick_o_effect: EffectContainer,
    pub quick_f_effect: EffectContainer,
    pub continuous_effect: EffectContainer,
    /// `oath_effects`: effect -> reason effect
    pub oath: Vec<(EffectId, Option<EffectId>)>,
    pub pheff: Vec<EffectId>,
    pub cheff: Vec<EffectId>,
    pub rechargeable: Vec<EffectId>,
    pub spsummon_count_eff: Vec<EffectId>,
    pub disable_check_set: CardSet,
    pub grant_effect: GrantEffects,
}

/// `field_info`
#[derive(Clone, Debug)]
pub struct FieldInfo {
    pub event_id: u32,
    pub field_id: u32,
    pub copy_id: u16,
    pub turn_id: i16,
    pub turn_id_by_player: [i16; 2],
    pub card_id: u32,
    pub phase: u16,
    pub turn_player: u8,
    pub priorities: [u8; 2],
    pub can_shuffle: bool,
}

impl Default for FieldInfo {
    fn default() -> Self {
        FieldInfo {
            event_id: 1,
            field_id: 1,
            copy_id: 1,
            turn_id: 0,
            turn_id_by_player: [0, 0],
            card_id: 1,
            phase: 0,
            turn_player: 0,
            priorities: [0, 0],
            can_shuffle: true,
        }
    }
}

/// `return_card_generic`
#[derive(Clone, Debug)]
pub struct ReturnCards<T> {
    pub canceled: bool,
    pub list: Vec<T>,
}

impl<T> Default for ReturnCards<T> {
    fn default() -> Self {
        ReturnCards { canceled: false, list: Vec::new() }
    }
}

impl<T> ReturnCards<T> {
    pub fn clear(&mut self) {
        self.canceled = false;
        self.list.clear();
    }
}

/// `processor::chain_limit_t`
#[derive(Clone, Debug)]
pub struct ChainLimit {
    pub function: LuaFn,
    pub player: i32,
}

/// `processor::action_value_t`
#[derive(Clone, Debug)]
pub struct ActionValue {
    pub check_function: Option<LuaFn>,
    pub player_amount: [u16; 2],
}

/// `processor::action_counter_t` (an unordered_map by counter id)
pub type ActionCounter = Vec<(u32, ActionValue)>;

/// `processor` (the `core` of the field).
#[derive(Clone, Debug, Default)]
pub struct Processor {
    pub units: VecDeque<Unit>,
    pub subunits: VecDeque<Unit>,
    pub reserved: Option<Unit>,
    pub just_sent_cards: CardSet,
    pub select_cards: Vec<CardId>,
    pub select_cards_codes: Vec<(u32, u32)>,
    pub unselect_cards: Vec<CardId>,
    pub summonable_cards: Vec<CardId>,
    pub spsummonable_cards: Vec<CardId>,
    pub repositionable_cards: Vec<CardId>,
    pub msetable_cards: Vec<CardId>,
    pub ssetable_cards: Vec<CardId>,
    pub attackable_cards: Vec<CardId>,
    pub select_effects: Vec<Option<EffectId>>,
    pub select_options: Vec<u64>,
    pub must_select_cards: Vec<CardId>,
    pub point_event: VecDeque<Event>,
    pub instant_event: VecDeque<Event>,
    pub queue_event: VecDeque<Event>,
    pub delayed_activate_event: VecDeque<Event>,
    pub full_event: VecDeque<Event>,
    pub used_event: VecDeque<Event>,
    pub single_event: VecDeque<Event>,
    pub solving_event: VecDeque<Event>,
    pub sub_solving_event: VecDeque<Event>,
    pub select_chains: VecDeque<Chain>,
    pub current_chain: Vec<Chain>,
    pub real_chain_count: i32,
    pub tpchain: VecDeque<Chain>,
    pub ntpchain: VecDeque<Chain>,
    pub ignition_priority_chains: VecDeque<Chain>,
    pub continuous_chain: VecDeque<Chain>,
    pub solving_continuous: VecDeque<Chain>,
    pub sub_solving_continuous: VecDeque<Chain>,
    pub delayed_continuous_tp: VecDeque<Chain>,
    pub delayed_continuous_ntp: VecDeque<Chain>,
    pub desrep_chain: VecDeque<Chain>,
    pub new_fchain: VecDeque<Chain>,
    pub new_fchain_s: VecDeque<Chain>,
    pub new_ochain: VecDeque<Chain>,
    pub new_ochain_s: VecDeque<Chain>,
    pub new_fchain_b: VecDeque<Chain>,
    pub new_ochain_b: VecDeque<Chain>,
    pub new_ochain_h: VecDeque<Chain>,
    pub new_chains: VecDeque<Chain>,
    /// `std::set<std::pair<effect*, tevent>>`
    pub delayed_quick_tmp: Vec<(EffectId, Event)>,
    pub delayed_quick: Vec<(EffectId, Event)>,
    /// `std::map<effect*, chain>`
    pub quick_f_chain: Vec<(EffectId, Chain)>,
    pub leave_confirmed: CardSet,
    pub special_summoning: CardSet,
    pub ss_tograve_set: CardSet,
    pub equiping_cards: CardSet,
    pub control_adjust_set: [CardSet; 2],
    pub unique_destroy_set: CardSet,
    pub self_destroy_set: CardSet,
    pub self_tograve_set: CardSet,
    pub trap_monster_adjust_set: [CardSet; 2],
    pub release_cards: CardSet,
    pub release_cards_ex: CardSet,
    pub release_cards_ex_oneof: CardSet,
    pub battle_destroy_rep: CardSet,
    pub fusion_materials: CardSet,
    pub operated_set: CardSet,
    pub discarded_set: CardSet,
    pub destroy_canceled: CardSet,
    pub delayed_enable_set: CardSet,
    pub set_group_pre_set: CardSet,
    pub set_group_set: CardSet,
    pub disfield_effects: Vec<EffectId>,
    pub extra_mzone_effects: Vec<EffectId>,
    pub extra_szone_effects: Vec<EffectId>,
    pub reseted_effects: Vec<EffectId>,
    pub readjust_map: Vec<(CardId, u32)>,
    pub unique_cards: [Vec<CardId>; 2],
    pub effect_count_code: Vec<(u64, u32)>,
    pub effect_count_code_duel: Vec<(u64, u32)>,
    pub effect_count_code_chain: Vec<(u64, u32)>,
    pub spsummon_once_map: [Vec<(u32, u32)>; 2],
    pub spsummon_once_map_rst: [Vec<(u32, u32)>; 2],
    /// `std::multimap<int32_t, card*, std::greater<int32_t>>`
    pub xmaterial_lst: Vec<(i32, CardId)>,
    pub global_flag: u32,
    pub pre_field: [u32; 2],
    pub opp_mzone: Vec<u32>,
    pub chain_limit: Vec<ChainLimit>,
    pub chain_limit_p: Vec<ChainLimit>,
    pub chain_solving: bool,
    pub conti_solving: bool,
    pub win_player: u8,
    pub win_reason: u8,
    pub re_adjust: bool,
    pub reason_effect: Option<EffectId>,
    pub reason_player: u8,
    pub summoning_card: Option<CardId>,
    pub summoning_proc_group_type: u32,
    pub summon_depth: u8,
    pub summon_cancelable: u8,
    pub attacker: Option<CardId>,
    pub attack_target: Option<CardId>,
    pub set_forced_attack: bool,
    pub forced_attacker: Option<CardId>,
    pub forced_attack_target: Option<CardId>,
    pub must_use_mats: Option<Group>,
    pub only_use_mats: Option<Group>,
    pub forced_summon_minc: i32,
    pub forced_summon_maxc: i32,
    pub attack_cancelable: bool,
    pub attack_cost_paid: u8,
    pub attack_rollback: bool,
    pub effect_damage_step: u8,
    pub battle_damage: [i32; 2],
    pub summon_count: [i32; 2],
    pub extra_summon: [u8; 2],
    pub spe_effect: [i32; 2],
    pub duel_options: u64,
    pub copy_reset: u32,
    pub copy_reset_count: u8,
    pub last_control_changed_id: u32,
    pub set_group_used_zones: u32,
    pub set_group_seq: [u8; 7],
    pub dice_results: Vec<u8>,
    pub coin_results: Vec<bool>,
    pub to_bp: bool,
    pub to_m2: bool,
    pub to_ep: bool,
    pub skip_m2: bool,
    pub chain_attack: bool,
    pub chain_attacker_id: u32,
    pub chain_attack_target: Option<CardId>,
    pub attack_player: u8,
    pub selfdes_disabled: bool,
    pub overdraw: [bool; 2],
    pub check_level: i32,
    pub shuffle_check_disabled: bool,
    pub shuffle_hand_check: [bool; 2],
    pub shuffle_deck_check: [bool; 2],
    pub deck_reversed: bool,
    pub remove_brainwashing: bool,
    pub flip_delayed: bool,
    pub damage_calculated: bool,
    pub hand_adjusted: bool,
    pub summon_state_count: [u8; 2],
    pub normalsummon_state_count: [u8; 2],
    pub flipsummon_state_count: [u8; 2],
    pub spsummon_state_count: [u8; 2],
    pub spsummon_state_count_rst: [u8; 2],
    pub spsummon_state_count_tmp: [u8; 2],
    pub spsummon_rst: bool,
    pub attack_state_count: [u8; 2],
    pub battle_phase_count: [u8; 2],
    pub battled_count: [u8; 2],
    pub phase_action: bool,
    pub hint_timing: [u32; 2],
    pub current_player: u8,
    pub conti_player: u8,
    pub force_turn_end: bool,
    pub summon_counter: ActionCounter,
    pub normalsummon_counter: ActionCounter,
    pub spsummon_counter: ActionCounter,
    pub flipsummon_counter: ActionCounter,
    pub attack_counter: ActionCounter,
    pub chain_counter: ActionCounter,
    pub recover_damage_reserve: VecDeque<Unit>,
    pub dec_count_reserve: Vec<EffectId>,
    // (patch)
    pub refreshing_condition: Option<EffectId>,
    pub set_group_seq_of: Vec<(CardId, u8)>,
    pub set_group_cur: Option<CardId>,
}

/// The options of a player (`OCG_Player`).
#[derive(Clone, Copy, Debug)]
pub struct PlayerOptions {
    pub starting_lp: u32,
    pub starting_draw_count: u32,
    pub draw_count_per_turn: u32,
}

/// `OCG_DuelOptions` (without the callbacks: card data comes from a `CardDb`).
#[derive(Clone, Debug)]
pub struct DuelOptions {
    pub seed: [u64; 4],
    pub flags: u64,
    pub team1: PlayerOptions,
    pub team2: PlayerOptions,
}

/// A duel: all of its state is plain data, so `clone()` copies it completely.
#[derive(Clone)]
pub struct Duel {
    // duel
    /// Cards and effects are shared between copies of the duel (copy-on-write: `card_mut` and
    /// `effect_mut` copy one only when another copy still uses it), so copying a duel is cheap
    /// (non-atomic counts: a duel stays on the thread it was made on).
    pub cards: Vec<Rc<Card>>,
    pub effects: Vec<Option<Rc<Effect>>>,
    /// `card::temp` for the stats, by card (see `StatTemp`)
    pub stat_temp: Vec<crate::card::StatTemp>,
    pub assumes: Vec<CardId>,
    /// Effects added while copying effects that are not copyable (`duel::uncopy`).
    pub uncopy: Vec<EffectId>,
    /// The groups shared by processor units (`GroupRef`); a slot is `None` when free.
    pub groups: Vec<Option<CardSet>>,
    pub db: Arc<CardDb>,
    pub messages: Vec<Message>,
    pub buff: Vec<u8>,
    pub query_buffer: Vec<u8>,
    pub random: Xoshiro256,
    pub querying: bool,
    pub mute_errors: bool,
    /// Script callbacks run in the current `ocg_process` call (see `api::SCRIPT_STEP_BUDGET`)
    pub script_steps: u64,
    pub muted_error: bool,
    // field
    pub player: [PlayerInfo; 2],
    pub temp_card: CardId,
    pub infos: FieldInfo,
    pub field_effects: FieldEffect,
    pub core: Processor,
    pub returns: ProgressiveBuffer,
    pub return_cards: ReturnCards<CardId>,
    pub return_card_codes: ReturnCards<(u32, u32)>,
    pub nil_event: Event,
    // interpreter
    pub params: Vec<Value>,
    /// Nesting depth of card functions run by `Execute*` units (`interpreter::call_depth`).
    pub call_depth: i32,
    /// `interpreter::no_action`
    pub no_action: i32,
    /// Nested calls of card functions from the engine (Lua's `nCcalls`: each `lua_pcall` of the
    /// core): at `LUA_MAX_C_CALLS` the call fails with "C stack overflow", as in Lua, instead of
    /// the native stack overflowing (e.g. two "Double Spell" asking about each other).
    pub lua_ccalls: u32,
    /// The card scripts (code -> initial_effect), shared by all copies.
    pub scripts: Arc<crate::script::Scripts>,
    /// Values kept by card scripts (the globals of the Lua scripts), by (code, key); shared by
    /// copies until one changes them.
    pub script_vars: Arc<Vec<((u32, u64), Value)>>,
    /// The replay scope of the card function being run (see DESIGN.md).
    pub scope: Option<Box<crate::scope::Scope>>,
    /// The extensions for the belief replay (`ext.rs`)
    pub ext: crate::ext::Ext,
    /// Stop inside card functions where the C++ core returns from `OCG_DuelProcess` (after each
    /// step that sent messages), so that a caller that changes the duel between `process` calls
    /// (the belief replay) does so at the same points (see `scope_stop_point`). Slower (a card
    /// function runs again from its start after each stop); off by default.
    pub fine_steps: bool,
    /// Log messages (`OCG_LogTypes`, text) of the core and the scripts (script errors are type 0),
    /// for the caller to take
    pub logs: Vec<(u32, String)>,
}

impl Duel {
    #[inline]
    pub fn card(&self, c: CardId) -> &Card {
        &self.cards[c.0 as usize]
    }
    #[inline]
    pub fn card_mut(&mut self, c: CardId) -> &mut Card {
        Rc::make_mut(&mut self.cards[c.0 as usize])
    }
    #[inline]
    pub fn effect(&self, e: EffectId) -> &Effect {
        self.effects[e.0 as usize].as_ref().expect("deleted effect")
    }
    #[inline]
    pub fn effect_mut(&mut self, e: EffectId) -> &mut Effect {
        Rc::make_mut(self.effects[e.0 as usize].as_mut().expect("deleted effect"))
    }
    pub fn effect_exists(&self, e: EffectId) -> bool {
        self.effects.get(e.0 as usize).is_some_and(|x| x.as_ref().is_some_and(|x| !x.deleted))
    }

    /// `duel::new_card` (with `interpreter::register_card`: the card's `initial_effect` runs,
    /// then the card gets its id).
    pub fn new_card(&mut self, code: u32) -> CardId {
        let id = CardId(self.cards.len() as u32);
        let mut c = Card::new();
        if code != 0 {
            c.data = self.read_card(code);
        }
        c.data.code = code;
        let (alias, script) = (c.data.alias, c.data.code);
        self.cards.push(Rc::new(c));
        self.stat_temp.push(Default::default());
        // (the script of an alternative artwork is the original's)
        let script = if alias != 0 && alias < script + 10 && script < alias + 10 { alias } else { script };
        self.card_mut(id).script_code = script;
        if code != 0 {
            self.card_mut(id).set_status(crate::constants::STATUS_INITIALIZING, true);
            self.call_initial_effect(script, id);
            self.card_mut(id).set_status(crate::constants::STATUS_INITIALIZING, false);
        }
        let cardid = self.infos.card_id;
        self.infos.card_id += 1;
        self.card_mut(id).cardid = cardid;
        debug_assert_eq!(cardid, id.0 + 1);
        id
    }
    /// `duel::new_effect`
    pub fn new_effect(&mut self) -> EffectId {
        let id = EffectId(self.effects.len() as u32);
        self.effects.push(Some(Rc::new(Effect::new())));
        id
    }
    /// `duel::delete_effect`
    pub fn delete_effect(&mut self, e: EffectId) {
        if let Some(x) = self.effects[e.0 as usize].as_mut() {
            Rc::make_mut(x).deleted = true;
        }
    }
    /// A new shared group (`duel::new_group` for the core's own groups).
    pub fn new_group(&mut self, set: CardSet) -> GroupRef {
        // (freed only between units, by `collect_groups`: a unit being processed holds its groups)
        if let Some(i) = self.groups.iter().position(|g| g.is_none()) {
            self.groups[i] = Some(set);
            return GroupRef(i as u32);
        }
        self.groups.push(Some(set));
        GroupRef(self.groups.len() as u32 - 1)
    }
    #[inline]
    pub fn group(&self, g: GroupRef) -> &CardSet {
        self.groups[g.0 as usize].as_ref().expect("freed group")
    }
    #[inline]
    pub fn group_mut(&mut self, g: GroupRef) -> &mut CardSet {
        self.groups[g.0 as usize].as_mut().expect("freed group")
    }
    pub fn group_opt(&self, g: Option<GroupRef>) -> Option<&CardSet> {
        g.map(|g| self.group(g))
    }
    /// Frees the shared groups no unit holds any more (called between units).
    pub fn collect_groups(&mut self) {
        if self.groups.iter().filter(|g| g.is_some()).count() < 32 {
            return;
        }
        let mut live = Vec::new();
        for u in self.core.units.iter().chain(self.core.subunits.iter()).chain(self.core.recover_damage_reserve.iter()).chain(self.core.reserved.iter()) {
            u.group_refs(&mut live);
        }
        let mut keep = vec![false; self.groups.len()];
        for g in live {
            keep[g.0 as usize] = true;
        }
        for (i, k) in keep.into_iter().enumerate() {
            if !k {
                self.groups[i] = None;
            }
        }
    }
    /// A value kept by card scripts (a Lua global or upvalue): `key` is (namespace, name).
    pub fn script_var(&self, key: (u32, u64)) -> Value {
        self.script_vars.iter().find(|x| x.0 == key).map_or(Value::Nil, |x| x.1.clone())
    }
    pub fn set_script_var(&mut self, key: (u32, u64), v: Value) {
        let vars = Arc::make_mut(&mut self.script_vars);
        match vars.iter_mut().find(|x| x.0 == key) {
            Some(x) => x.1 = v,
            None => vars.push((key, v)),
        }
    }
    /// `duel::read_card`
    pub fn read_card(&self, code: u32) -> CardData {
        self.db.get(code)
    }
    /// `duel::new_message`: the index of the new message (write to it with `msg`).
    pub fn new_message(&mut self, message: u32) -> usize {
        let mut m = Message { data: Vec::with_capacity(16) };
        m.data.push(message as u8);
        self.messages.push(m);
        self.messages.len() - 1
    }
    #[inline]
    pub fn msg(&mut self, i: usize) -> &mut Message {
        &mut self.messages[i]
    }
    /// `duel::generate_buffer`: moves the pending messages to `buff` (length-prefixed).
    pub fn generate_buffer(&mut self) {
        let messages = std::mem::take(&mut self.messages);
        for m in messages {
            if m.data.is_empty() {
                continue;
            }
            if let Some(scope) = self.scope.as_mut() {
                if scope.skip_messages > 0 {
                    scope.skip_messages -= 1;
                    continue;
                }
                scope.delivered_messages += 1;
            }
            self.buff.extend_from_slice(&(m.data.len() as u32).to_le_bytes());
            self.buff.extend_from_slice(&m.data);
        }
    }
    /// Makes a change from outside (an API call, e.g. a forcing of the belief replay): now, and if
    /// a card function waits for a response, again at the same point when it runs again from its
    /// snapshot (else the change would be lost). Returns `f`'s result.
    pub fn outside_change<T>(&mut self, f: impl Fn(&mut Duel) -> T + Send + Sync + 'static) -> T {
        let f = std::sync::Arc::new(f);
        let r = f(self);
        if let Some(s) = self.scope.as_mut() {
            if !s.running {
                let g = f.clone();
                s.ops.push((s.waiting_at, std::sync::Arc::new(move |d: &mut Duel| {
                    g(d);
                })));
            }
        }
        r
    }
    /// `duel::set_response`
    pub fn set_response(&mut self, resp: &[u8]) {
        if let Some(s) = self.scope.as_mut() {
            if !s.running {
                s.log.push(resp.to_vec());
                return;
            }
        }
        self.returns.data.clear();
        self.returns.data.extend_from_slice(resp);
    }
    /// `duel::get_next_integer`: uniform integer in [l, h].
    pub fn get_next_integer(&mut self, l: i32, h: i32) -> i32 {
        debug_assert!(l <= h);
        let range = (h as i64 - l as i64 + 1) as u64;
        let lim = u64::MAX % range;
        let mut n;
        loop {
            n = self.random.next();
            if n > lim {
                break;
            }
        }
        ((n % range) as i64 + l as i64) as i32
    }
    /// `duel::restore_assumes`
    pub fn restore_assumes(&mut self) {
        for c in std::mem::take(&mut self.assumes) {
            self.card_mut(c).assume.clear();
        }
    }
    /// `field::is_flag`
    #[inline]
    pub fn is_flag(&self, flag: u64) -> bool {
        self.core.duel_options & flag == flag
    }
    /// A log message (`duel::handle_message`); errors of card functions end up here.
    pub fn handle_message(&mut self, message: &str, typ: u32) {
        if self.mute_errors && typ == 0 {
            self.muted_error = true;
            return;
        }
        // (as messages: a scope run again does not log again what an earlier run logged)
        if let Some(s) = self.scope.as_mut().filter(|s| s.running) {
            if s.skip_logs > 0 {
                s.skip_logs -= 1;
                return;
            }
            s.delivered_logs += 1;
        }
        // (kept for the caller, e.g. to count script errors; at most a few, not to grow a copy)
        if self.logs.len() < 1000 {
            self.logs.push((typ, message.to_string()));
        }
    }

    /// `duel::duel` + `field::field`
    pub fn new(options: &DuelOptions, db: Arc<CardDb>, scripts: Arc<crate::script::Scripts>) -> Duel {
        let mut core = Processor::default();
        core.duel_options = options.flags;
        core.win_player = 5;
        core.reason_player = PLAYER_NONE as u8;
        core.conti_player = PLAYER_NONE as u8;
        let nil_event = Event {
            event_code: 0,
            event_cards: None,
            event_player: PLAYER_NONE as u8,
            event_value: 0,
            reason: 0,
            reason_effect: None,
            reason_player: PLAYER_NONE as u8,
            ..Default::default()
        };
        let mut d = Duel {
            cards: Vec::with_capacity(128),
            effects: Vec::with_capacity(256),
            stat_temp: Vec::with_capacity(128),
            assumes: Vec::new(),
            uncopy: Vec::new(),
            groups: Vec::new(),
            db,
            messages: Vec::new(),
            buff: Vec::new(),
            query_buffer: Vec::new(),
            random: Xoshiro256::new(options.seed),
            querying: false,
            mute_errors: false,
            script_steps: 0,
            muted_error: false,
            player: [PlayerInfo::new(&options.team1), PlayerInfo::new(&options.team2)],
            temp_card: CardId(0),
            infos: FieldInfo::default(),
            field_effects: FieldEffect::default(),
            core,
            returns: ProgressiveBuffer::default(),
            return_cards: ReturnCards::default(),
            return_card_codes: ReturnCards::default(),
            nil_event,
            params: Vec::new(),
            call_depth: 0,
            no_action: 0,
            scripts,
            script_vars: Arc::new(Vec::new()),
            scope: None,
            ext: Default::default(),
            lua_ccalls: 0,
            fine_steps: false,
            logs: Vec::new(),
        };
        d.temp_card = d.new_card(0);
        d
    }
}

/// Uses the Lua-convention boolean of ygopro-core (`TRUE`/`FALSE` as ints).
#[inline]
pub fn b2i(b: bool) -> i32 {
    b as i32
}

#[cfg(test)]
mod copy_tests {
    use super::*;

    fn duel() -> Duel {
        let p = PlayerOptions { starting_lp: 8000, starting_draw_count: 5, draw_count_per_turn: 1 };
        let options = DuelOptions { seed: [1, 2, 3, 4], flags: 0, team1: p, team2: p };
        let mut d = Duel::new(&options, Arc::new(CardDb::default()), Arc::new(crate::script::Scripts::default()));
        for _ in 0..40 {
            d.new_card(0);
        }
        for _ in 0..40 {
            d.new_effect();
        }
        d
    }

    /// A copy of a duel shares its cards and effects (copying them is what made copies slow),
    /// and writing to either copy leaves the other unchanged.
    #[test]
    fn copies_share_cards_and_stay_independent() {
        let mut a = duel();
        let n = (a.cards.len(), a.effects.len());
        let mut b = a.clone();
        assert!(a.cards.iter().zip(&b.cards).all(|(x, y)| Rc::ptr_eq(x, y)), "a copy deep-copied the cards");
        let shared = |a: &Duel, b: &Duel| a.effects.iter().zip(&b.effects).all(|(x, y)| match (x, y) {
            (Some(x), Some(y)) => Rc::ptr_eq(x, y),
            _ => false,
        });
        assert!(shared(&a, &b), "a copy deep-copied the effects");

        let (c, e) = (CardId(3), EffectId(5));
        b.card_mut(c).data.attack = 1234;
        b.effect_mut(e).label.push(7);
        a.card_mut(CardId(4)).data.attack = 99;
        assert_eq!(a.card(c).data.attack, 0);
        assert!(a.effect(e).label.is_empty());
        assert_eq!(b.card(c).data.attack, 1234);
        assert_eq!(b.effect(e).label, vec![7]);
        assert_eq!(b.card(CardId(4)).data.attack, 0);
        // (only the written cards and effects were copied)
        assert!(!Rc::ptr_eq(&a.cards[3], &b.cards[3]) && !Rc::ptr_eq(&a.cards[4], &b.cards[4]));
        assert!(Rc::ptr_eq(&a.cards[5], &b.cards[5]));

        // new cards and effects in one copy do not appear in the other
        b.new_card(0);
        b.new_effect();
        assert_eq!((a.cards.len(), a.effects.len()), n);
        drop(b);
        assert_eq!(a.card(c).data.attack, 0);
    }
}
