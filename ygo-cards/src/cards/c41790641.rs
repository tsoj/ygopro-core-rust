//! Wattkingdom
use super::prelude::*;

pub const ID: u32 = 41790641;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WATT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Prevent the activation of some monster's effects in this card's column and the column of your "Watt" monster
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, 1);
    effect::set_value_fn(d, e2, lua_fn(actlimit));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Watt" monster from the Deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn wattfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_WATT as u16]) && card::is_faceup(d, c) && card::is_controler(d, c, tp)
}
/// `function(e,re,tp)`
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    let rc = effect::get_handler(d, re);
    let efftyp = effect::get_type(d, re);
    let effcod = effect::get_code(d, re);
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if !card::is_on_field(d, rc.nil()) {
        return false;
    }
    let rc = rc.nil();
    let hp = effect::get_handler_player(d, e);
    let column_hit = {
        let hg = card::get_column_group(d, effect::get_handler(d, e).nil(), 0, 0);
        group::is_contains(&hg, rc)
    } || {
        let rg = card::get_column_group(d, rc, 0, 0);
        group::is_exists(d, &rg, &|d, c| wattfilter(d, c, hp), 1, NO_EX)
    };
    // Lua: (efftyp&EFFECT_TYPE_TRIGGER_O)>0 or (efftyp&EFFECT_TYPE_TRIGGER_F)
    // The second operand is a number, which is always true in Lua.
    column_hit
        && (efftyp & EFFECT_TYPE_SINGLE) > 0
        && (((efftyp & EFFECT_TYPE_TRIGGER_O) > 0) || { let _trigger_f = efftyp & EFFECT_TYPE_TRIGGER_F; true })
        && (effcod == EVENT_SUMMON_SUCCESS || effcod == EVENT_SPSUMMON_SUCCESS)
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_WATT as u16])
        && {
            let code = card::get_code(d, c)[0];
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    card::is_set_card(d, c, &[SET_WATT as u16])
        && !card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, a.e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let _g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon, except Thunder monsters, for the rest of the turn
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_THUNDER as u64))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // Special Summon 1 "Watt" monster with a different name from the Deck
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let code = card::get_code(d, tc)[0];
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            let atk = card::get_attack(d, sc);
            if atk > 0 {
                duel::break_effect(d);
                let lp = duel::get_lp(d, tp);
                duel::set_lp(d, tp, lp - atk);
            }
        }
    }
}
