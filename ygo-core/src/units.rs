//! The processor units of ygopro-core (`processor_unit.h`): resumable steps of the rules engine.
//! Each unit has a `step`; `Duel::process` runs the first unit until it returns `true` (done).
//! Units that ask a player (`needs_answer`) return `false` after sending the question.

use crate::types::{CardId, CardSet, EffectId, GroupRef};

pub const RESTART: u16 = u16::MAX;

/// `LOCATION_REASON`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LocationReason {
    None = 0x0,
    ToField = 0x1,
    Control = 0x2,
    Count = 0x4,
    Return = 0x8,
    ReturnToField = 0x9,
    Summon = 0x10,
    SpSummon = 0x11,
    Set = 0x12,
}

macro_rules! units {
    ($( $name:ident ($na:expr) { $($field:ident : $ty:ty),* $(,)? } )*) => {
        $(
            #[derive(Clone, Debug)]
            pub struct $name { pub step: u16, $(pub $field: $ty),* }
            impl From<$name> for Unit { fn from(u: $name) -> Unit { Unit::$name(u) } }
        )*
        #[derive(Clone, Debug)]
        pub enum Unit { $($name($name)),* }
        impl Unit {
            pub fn step(&self) -> u16 { match self { $(Unit::$name(u) => u.step),* } }
            pub fn step_mut(&mut self) -> &mut u16 { match self { $(Unit::$name(u) => &mut u.step),* } }
            pub fn needs_answer(&self) -> bool { match self { $(Unit::$name(_) => $na),* } }
            pub fn name(&self) -> &'static str { match self { $(Unit::$name(_) => stringify!($name)),* } }
            /// The shared groups this unit holds (for `Duel::collect_groups`).
            pub fn group_refs(&self, out: &mut Vec<GroupRef>) {
                #[allow(unused_variables)]
                match self { $(Unit::$name(u) => { $( push_ref(&u.$field, out); )* }),* }
            }
        }
    };
}

units! {
    Adjust(false) {}
    Turn(false) { turn_player: u8, has_performed_second_battle_phase: bool }
    RefreshLoc(false) { dis_count: u8, previously_disabled_locations: u32, current_disable_field_effect: Option<EffectId> }
    Startup(false) {}
    SelectBattleCmd(true) { playerid: u8 }
    SelectIdleCmd(true) { playerid: u8 }
    SelectEffectYesNo(true) { playerid: u8, pcard: Option<CardId>, description: u64 }
    SelectYesNo(true) { playerid: u8, description: u64 }
    SelectOption(true) { playerid: u8 }
    SelectCard(true) { playerid: u8, cancelable: bool, min: u8, max: u8 }
    SelectCardCodes(true) { playerid: u8, cancelable: bool, min: u8, max: u8 }
    SelectUnselectCard(true) { playerid: u8, cancelable: bool, min: u8, max: u8, finishable: bool }
    SelectChain(true) { playerid: u8, spe_count: u8, forced: bool }
    SelectPlace(true) { playerid: u8, count: u8, flag: u32, disable_field: bool }
    SelectPosition(true) { playerid: u8, positions: u8, code: u32 }
    SelectTributeP(true) { playerid: u8, cancelable: bool, min: u8, max: u8 }
    SortChain(false) { playerid: u8 }
    SelectCounter(true) { countertype: u16, count: u16, playerid: u8, self_: u8, oppo: u8 }
    SelectSum(true) { playerid: u8, acc: i32, min: i32, max: i32 }
    SortCard(true) { playerid: u8, is_chain: bool }
    SelectRelease(false) { min: u16, max: u16, playerid: u8, cancelable: bool, check_field: bool, toplayer: u8, zone: u8,
        to_check: Option<CardId>, extra_release_nonsum_effect: Option<EffectId>, must_choose_one: Option<CardSet> }
    SelectTribute(false) { min: u16, max: u16, playerid: u8, cancelable: bool, toplayer: u8, zone: u8,
        target: Option<CardId>, extra_release_effect: Option<EffectId>, must_choose_one: CardSet }
    QuickEffect(false) { skip_freechain: bool, is_opponent: bool, priority_player: u8 }
    IdleCommand(false) { phase_to_change_to: u8, card_to_reposition: Option<CardId> }
    PhaseEvent(false) { phase: u16, is_opponent: bool, priority_passed: bool }
    PointEvent(false) { skip_trigger: bool, skip_freechain: bool, skip_new: bool }
    BattleCommand(false) { phase_to_change_to: u16, forced_attack: bool, forced_attack_done: bool, is_replaying_attack: bool,
        attack_announce_failed: bool, repeat_battle_phase: bool, second_battle_phase_is_optional: bool,
        previous_point_event_had_any_trigger_to_resolve: bool, reason_player: u8, damage_change_effect: Option<EffectId>,
        cards_destroyed_by_battle: Option<GroupRef>, reason_card: Option<CardId>, must_attack_map: Vec<(EffectId, CardId)> }
    DamageStep(false) { backup_phase: u16, new_attack: bool, attacker: Option<CardId>, attack_target: Option<CardId>,
        cards_destroyed_by_battle: Option<GroupRef> }
    ForcedBattle(false) { backup_phase: u16 }
    AddChain(false) { is_activated_effect: bool }
    SolveChain(false) { skip_trigger: bool, skip_freechain: bool, skip_new: bool, backed_up_operation: Option<crate::types::LuaFn> }
    SolveContinuous(false) { reason_player: u8, reason_effect: Option<EffectId> }
    ExecuteCost(false) { triggering_player: u8, shuffle_check_was_disabled: bool, triggering_effect: EffectId }
    ExecuteOperation(false) { triggering_player: u8, shuffle_check_was_disabled: bool, triggering_effect: EffectId }
    ExecuteTarget(false) { triggering_player: u8, shuffle_check_was_disabled: bool, triggering_effect: EffectId }
    Destroy(false) { reason_player: u8, reason: u32, targets: Option<GroupRef>, reason_effect: Option<EffectId> }
    Release(false) { reason_player: u8, reason: u32, targets: Option<GroupRef>, reason_effect: Option<EffectId> }
    SendTo(false) { reason_player: u8, reason: u32, targets: Option<GroupRef>, reason_effect: Option<EffectId>, extra_args: Option<Box<SendToExargs>> }
    DestroyReplace(false) { battle: bool, targets: Option<GroupRef>, target: CardId }
    ReleaseReplace(false) { targets: Option<GroupRef>, target: CardId }
    SendToReplace(false) { targets: Option<GroupRef>, target: CardId }
    MoveToField(false) { enable: bool, ret: u8, pzone: bool, zone: u8, rule: bool, location_reason: LocationReason, confirm: bool, target: CardId }
    ChangePos(false) { reason_player: u8, enable: bool, oppo_selection: bool, reason_effect: Option<EffectId>, targets: Option<GroupRef>, to_grave_set: CardSet }
    OperationReplace(false) { is_destroy: bool, replace_effect: Option<EffectId>, targets: Option<GroupRef>, target: Option<CardId> }
    ActivateEffect(false) { peffect: EffectId }
    SummonRule(false) { sumplayer: u8, min_tribute: u8, max_allowed_tributes: u8, ignore_count: bool, zone: u32, target: CardId,
        summon_procedure_effect: Option<EffectId>, extra_summon_effect: Option<EffectId>, tributes: CardSet, summon_cost_effects: Vec<EffectId> }
    SpSummonRule(false) { sumplayer: u8, is_mid_chain: bool, summon_type: u32, target: CardId, summon_proc_effect: Option<EffectId>,
        cards_to_summon_g: Option<GroupRef>, spsummon_cost_effects: Vec<EffectId>, iter_order: Vec<CardId>, iter_pos: usize }
    SpSummon(false) { reason_player: u8, zone: u32, reason_effect: Option<EffectId>, targets: Option<GroupRef> }
    FlipSummon(false) { sumplayer: u8, target: CardId, flip_summon_cost_effects: Vec<EffectId> }
    MonsterSet(false) { setplayer: u8, min_tribute: u8, max_allowed_tributes: u8, ignore_count: bool, zone: u32, target: CardId,
        summon_procedure_effect: Option<EffectId>, extra_summon_effect: Option<EffectId>, tributes: CardSet }
    SpellSet(false) { setplayer: u8, toplayer: u8, target: CardId, reason_effect: Option<EffectId> }
    SpSummonStep(false) { zone: u32, targets: Option<GroupRef>, target: CardId, spsummon_cost_effects: Vec<EffectId> }
    SpellSetGroup(false) { setplayer: u8, toplayer: u8, confirm: bool, ptarget: Option<GroupRef>, reason_effect: Option<EffectId>, set_cards: CardSet }
    SpSummonRuleGroup(false) { sumplayer: u8, summon_type: u32 }
    Draw(false) { count: u16, reason_player: u8, playerid: u8, reason: u32, reason_effect: Option<EffectId>, drawn_set: CardSet }
    Damage(false) { reason_player: u8, playerid: u8, is_step: bool, is_reflected: bool, amount: u32, reason: u32,
        reason_card: Option<CardId>, reason_effect: Option<EffectId> }
    Recover(false) { reason_player: u8, playerid: u8, is_step: bool, amount: u32, reason: u32, reason_effect: Option<EffectId> }
    Equip(false) { equip_player: u8, is_step: bool, faceup: bool, equip_card: CardId, target: CardId }
    GetControl(false) { chose_player: u8, playerid: u8, reset_count: u8, reset_phase: u16, zone: u32, reason_effect: Option<EffectId>,
        targets: Option<GroupRef>, destroy_set: CardSet, iter_order: Vec<CardId>, iter_pos: usize }
    SwapControl(false) { reset_count: u8, reason_player: u8, self_selected_sequence: u8, reset_phase: u16, reason_effect: Option<EffectId>,
        targets1: Option<GroupRef>, targets2: Option<GroupRef>, iter1: Vec<CardId>, pos1: usize, iter2: Vec<CardId>, pos2: usize }
    ControlAdjust(false) { adjusting_player: u8, destroy_set: CardSet, adjust_set: CardSet }
    SelfDestroyUnique(false) { playerid: u8, unique_card: CardId }
    SelfDestroy(false) {}
    SelfToGrave(false) {}
    TrapMonsterAdjust(false) { oppo_selection: bool, to_grave_set: CardSet }
    PayLPCost(false) { playerid: u8, cost: u32 }
    RemoveCounter(false) { rplayer: u8, self_: u8, oppo: u8, countertype: u16, count: u16, reason: u32, pcard: Option<CardId> }
    AttackDisable(false) {}
    AnnounceRace(true) { playerid: u8, count: u8, available: u64 }
    AnnounceAttribute(true) { playerid: u8, count: u8, available: u32 }
    AnnounceCard(true) { playerid: u8 }
    AnnounceNumber(true) { playerid: u8 }
    TossCoin(false) { playerid: u8, reason_player: u8, count: u8, reason_effect: Option<EffectId> }
    TossDice(false) { playerid: u8, reason_player: u8, count1: u8, count2: u8, reason_effect: Option<EffectId> }
    RockPaperScissors(true) { repeat: bool, hand0: u8 }
    SelectFusion(false) { playerid: u8, chkf: u64, fusion_materials: Option<GroupRef>, forced_materials: Option<GroupRef>, pcard: Option<CardId> }
    DiscardHand(false) { playerid: u8, min: u8, max: u8, reason: u32 }
    DiscardDeck(false) { count: u16, playerid: u8, reason: u32 }
    SortDeck(false) { count: u16, sort_player: u8, target_player: u8, bottom: bool }
    RemoveOverlay(false) { min: u16, max: u16, replaced_amount: u16, has_used_overlay_remove_replace_effect: bool, rplayer: u8,
        self_: u8, oppo: u8, reason: u32, pgroup: Option<GroupRef> }
    XyzOverlay(false) { send_materials_to_grave: bool, target: CardId, materials: Option<GroupRef> }
    RefreshRelay(false) {}
}

/// `SendTo::exargs`
#[derive(Clone, Debug, Default)]
pub struct SendToExargs {
    pub leave_field: CardSet,
    pub leave_grave: CardSet,
    pub detach: CardSet,
    pub check_decktop_visibility: [bool; 2],
    pub cv: Vec<CardId>,
    pub cvit: usize,
    pub predirect: Option<EffectId>,
}

/// Collects a `GroupRef` field (other field types hold no shared group).
fn push_ref<T: std::any::Any>(f: &T, out: &mut Vec<GroupRef>) {
    let a = f as &dyn std::any::Any;
    if let Some(Some(g)) = a.downcast_ref::<Option<GroupRef>>() {
        out.push(*g);
    } else if let Some(g) = a.downcast_ref::<GroupRef>() {
        out.push(*g);
    }
}
