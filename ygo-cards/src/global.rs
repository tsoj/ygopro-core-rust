//! The rule effects the global scripts register when they are loaded, in load order
//! (`utility.lua` loads `chain.lua`, ..., `proc_fusion_spell.lua`, ..., `proc_workaround.lua`,
//! `proc_normal.lua`, `proc_skill.lua`, `proc_rush.lua`, `proc_maximum.lua`, ...).

use crate::constants::*;
use crate::lua::{card, duel, effect, group};
use crate::prelude::*;

/// `proc_maximum.lua`
pub const FLAG_MAXIMUM_CENTER: u32 = 170000000;
pub const FLAG_MAXIMUM_SIDE: u32 = 170000001;

/// Script state of the global scripts (`Duel::script_var` keys).
pub mod vars {
    use crate::ns::GLOBAL;
    /// `Fusion.ExtraGroup` (proc_fusion_spell.lua)
    pub const FUSION_EXTRA_GROUP: (u32, u64) = (GLOBAL, 1);
    /// `player_table` of the hand shuffle (proc_workaround.lua): [p0, p1] as bits 0/1.
    pub const SHUFFLED_PLAYERS: (u32, u64) = (GLOBAL, 2);
}

pub fn load(d: &mut Duel) {
    proc_fusion_spell(d);
    proc_workaround(d);
    proc_skill(d);
    if duel::is_duel_type(d, DUEL_INVERTED_QUICK_PRIORITY as u64) {
        unimplemented!("proc_rush.lua: Rush Duel rules");
    }
    proc_maximum(d);
}

fn proc_fusion_spell(d: &mut Duel) {
    let geff = effect::global_effect(d);
    effect::set_type(d, geff, EFFECT_TYPE_FIELD);
    effect::set_code(d, geff, EFFECT_EXTRA_FUSION_MATERIAL);
    effect::set_property(d, geff, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, geff, 0xff, 0xff);
    effect::set_target(
        d,
        geff,
        Some(effect_card_fn(|d, _e, c| match d.script_var(vars::FUSION_EXTRA_GROUP) {
            Value::Group(g) => g.contains(c),
            _ => false,
        })),
    );
    effect::set_value_fn(d, geff, aux_true());
    duel::register_effect(d, geff, 0);
}

fn shuffled(d: &Duel, p: u8) -> bool {
    d.script_var(vars::SHUFFLED_PLAYERS).int() & (1 << p) != 0
}

fn proc_workaround(d: &mut Duel) {
    // Allow only 1 Chain to be built if an effect meets its activation condition during hand
    // size adjustment at the end of the End Phase
    let hand_adjust_eff = effect::global_effect(d);
    effect::set_type(d, hand_adjust_eff, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, hand_adjust_eff, EVENT_DISCARD);
    effect::set_count_limit(d, hand_adjust_eff, 1, 0, 0, 0);
    effect::set_condition(d, hand_adjust_eff, Some(condition(|_d, a| a.r & REASON_ADJUST > 0)));
    effect::set_operation(
        d,
        hand_adjust_eff,
        Some(operation(|d, _a| {
            // Track the moment that the Chain starts resolving
            let chain_solving_eff = effect::global_effect(d);
            effect::set_type(d, chain_solving_eff, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, chain_solving_eff, EVENT_CHAIN_SOLVING);
            effect::set_count_limit(d, chain_solving_eff, 1, 0, 0, 0);
            effect::set_operation(
                d,
                chain_solving_eff,
                Some(operation(|d, a| {
                    effect::reset(d, a.e);
                    // Neither player can activate another effect after the first Chain built
                    // during hand size adjustment starts resolving
                    let cannot_act_eff = effect::global_effect(d);
                    effect::set_type(d, cannot_act_eff, EFFECT_TYPE_FIELD);
                    effect::set_property(d, cannot_act_eff, EFFECT_FLAG_PLAYER_TARGET, 0);
                    effect::set_code(d, cannot_act_eff, EFFECT_CANNOT_ACTIVATE);
                    effect::set_target_range(d, cannot_act_eff, 1, 1);
                    effect::set_value(d, cannot_act_eff, 1);
                    effect::set_reset(d, cannot_act_eff, RESET_PHASE | PHASE_END, 0);
                    duel::register_effect(d, cannot_act_eff, 0);
                })),
            );
            effect::set_reset(d, chain_solving_eff, RESET_PHASE | PHASE_END, 0);
            duel::register_effect(d, chain_solving_eff, 0);
        })),
    );
    duel::register_effect(d, hand_adjust_eff, 0);

    // If a monster in the Monster Zone is flipped face-down and back up again it shouldn't be
    // treated as a monster that was Summoned that turn
    let sum_status_eff = effect::global_effect(d);
    effect::set_type(d, sum_status_eff, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, sum_status_eff, EVENT_CHANGE_POS);
    effect::set_operation(
        d,
        sum_status_eff,
        Some(operation(|d, a| {
            let filter = |d: &mut Duel, c: CardId| {
                card::is_facedown(d, c)
                    && card::is_previous_position(d, c, POS_FACEUP)
                    && card::is_location(d, c, LOCATION_MZONE)
                    && card::is_status(d, c, STATUS_SUMMON_TURN | STATUS_FLIP_SUMMON_TURN | STATUS_SPSUMMON_TURN)
            };
            let g = group::filter(d, &a.eg(), &filter, crate::lua::NO_EX);
            for c in group::iter(d, &g) {
                card::set_status(d, c, STATUS_SUMMON_TURN, false);
                card::set_status(d, c, STATUS_FLIP_SUMMON_TURN, false);
                card::set_status(d, c, STATUS_SPSUMMON_TURN, false);
                card::set_status(d, c, STATUS_FORM_CHANGED, true);
            }
        })),
    );
    duel::register_effect(d, sum_status_eff, 0);

    // set the summon turn status to 'false' for any monster that is Normal Set
    let set_turn_status_split = effect::global_effect(d);
    effect::set_type(d, set_turn_status_split, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, set_turn_status_split, EVENT_MSET);
    effect::set_operation(
        d,
        set_turn_status_split,
        Some(operation(|d, a| {
            for c in group::iter(d, &a.eg()) {
                card::set_status(d, c, STATUS_SUMMON_TURN, false);
                card::set_status(d, c, STATUS_FORM_CHANGED, true);
            }
        })),
    );
    duel::register_effect(d, set_turn_status_split, 0);

    // Automatically shuffle a player's hand when the effect of a card that they activated in the
    // hand begins resolving, but only if that same card is still in the hand on resolution
    let shuffle_eff = effect::global_effect(d);
    effect::set_type(d, shuffle_eff, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, shuffle_eff, EVENT_CHAIN_SOLVING);
    effect::set_operation(d, shuffle_eff, Some(operation(shuffle_op)));
    duel::register_effect(d, shuffle_eff, 0);

    // reset the player tracking at the end of each Chain
    let tracking_eff = effect::global_effect(d);
    effect::set_type(d, tracking_eff, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, tracking_eff, EVENT_CHAIN_END);
    effect::set_operation(d, tracking_eff, Some(operation(|d, _a| d.set_script_var(vars::SHUFFLED_PLAYERS, Value::Int(0)))));
    duel::register_effect(d, tracking_eff, 0);

    // Registers a flag effect on each monster that is Normal or Special Summoned, with the
    // current phase being the flag effect's label ("Wicked Dragon of Darkness")
    let ns_eff = effect::global_effect(d);
    effect::set_type(d, ns_eff, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, ns_eff, EVENT_SUMMON_SUCCESS);
    effect::set_operation(
        d,
        ns_eff,
        Some(operation(|d, a| {
            for sc in group::iter(d, &a.eg()) {
                let ph = duel::get_current_phase(d);
                card::register_flag_effect(d, sc, 160214042, RESET_EVENT | RESETS_STANDARD, 0, 1, ph as i64, 0);
            }
        })),
    );
    duel::register_effect(d, ns_eff, 0);
    let sp_eff = effect::clone(d, ns_eff);
    effect::set_code(d, sp_eff, EVENT_SPSUMMON_SUCCESS);
    duel::register_effect(d, sp_eff, 0);
}

/// The hand shuffle when an effect activated in the hand starts resolving.
fn shuffle_op(d: &mut Duel, a: &EffArgs) {
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]);
    if loc.first().map(|v| v.int()) != Some(LOCATION_HAND as i64) {
        return;
    }
    let re = a.re.unwrap();
    let rc = effect::get_handler(d, re).unwrap();
    let player = card::get_controler(d, rc);
    // if this player's hand was already shuffled earlier in this Chain then don't shuffle it again
    if shuffled(d, player) {
        return;
    }
    // if it's not the same card in the hand anymore then don't shuffle
    if !(card::is_relate_to_effect(d, rc, re) && card::is_location(d, rc, LOCATION_HAND)) {
        return;
    }
    // if there's opinfo that would (even potentially) move rc then don't shuffle
    let check_opinfo = |d: &mut Duel, cat: u64| {
        let has = |o: Option<ygo_core::duel::OpTarget>| o.is_some_and(|o| o.op_cards.is_some_and(|g| g.contains(rc)));
        has(duel::get_operation_info(d, a.ev, cat)) || has(duel::get_possible_operation_info(d, a.ev, cat))
    };
    for cat in [CATEGORY_SPECIAL_SUMMON, CATEGORY_SUMMON, CATEGORY_TOGRAVE, CATEGORY_DESTROY, CATEGORY_REMOVE, CATEGORY_TODECK, CATEGORY_TOEXTRA, CATEGORY_EQUIP, CATEGORY_RELEASE] {
        if check_opinfo(d, cat as u64) {
            return;
        }
    }
    duel::shuffle_hand(d, player);
    // if the activating card itself ends up moving then shuffle the hand after the current Chain
    // Link finishes resolving
    let move_eff = effect::create_effect(d, rc);
    effect::set_type(d, move_eff, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, move_eff, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, move_eff, EVENT_MOVE);
    effect::set_operation(
        d,
        move_eff,
        Some(operation(move |d, a| {
            let eff = effect::create_effect(d, rc);
            effect::set_type(d, eff, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, eff, EVENT_CHAIN_SOLVED);
            effect::set_operation(
                d,
                eff,
                Some(operation(move |d, a| {
                    duel::shuffle_hand(d, player);
                    effect::reset(d, a.e);
                })),
            );
            effect::set_reset(d, eff, RESET_CHAIN, 0);
            duel::register_effect(d, eff, player);
            effect::reset(d, a.e);
        })),
    );
    effect::set_reset(d, move_eff, RESET_CHAIN, 0);
    card::register_effect(d, rc, move_eff, false);
    let v = d.script_var(vars::SHUFFLED_PLAYERS).int() | (1 << player);
    d.set_script_var(vars::SHUFFLED_PLAYERS, Value::Int(v));
}

fn proc_skill(d: &mut Duel) {
    let drawlesseff = effect::global_effect(d);
    effect::set_type(d, drawlesseff, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, drawlesseff, EVENT_STARTUP);
    effect::set_operation(
        d,
        drawlesseff,
        Some(operation(|d, a| {
            // (aux.drawlessop: Skill cards that make their player draw fewer cards; none here)
            effect::reset(d, a.e);
        })),
    );
    duel::register_effect(d, drawlesseff, 0);
}

fn is_maximum_mode(d: &Duel, c: CardId) -> bool {
    card::has_flag_effect(d, c, FLAG_MAXIMUM_CENTER, None) || card::has_flag_effect(d, c, FLAG_MAXIMUM_SIDE, None)
}

fn proc_maximum(d: &mut Duel) {
    let e1 = effect::global_effect(d);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_FORCE_SPSUMMON_POSITION);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| is_maximum_mode(d, c))));
    effect::set_value(d, e1, POS_FACEUP_ATTACK as i64);
    duel::register_effect(d, e1, 0);
    let e2 = effect::global_effect(d);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_FORCE_MZONE);
    effect::set_target_range(d, e2, 0xff, 0xff);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| is_maximum_mode(d, c))));
    effect::set_value_fn(
        d,
        e2,
        effect_card_value(|d, _e, c| {
            if card::has_flag_effect(d, c, FLAG_MAXIMUM_CENTER, None) {
                0x4
            } else if crate::script_info(card::get_original_code(d, c)).maximum_side == Some(MaximumSide::Left) {
                0x2
            } else {
                0x8
            }
        }),
    );
    duel::register_effect(d, e2, 0);
}

/// `MaximumSide` of a card's script ("Left"/"Right").
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MaximumSide {
    Left,
    Right,
}
