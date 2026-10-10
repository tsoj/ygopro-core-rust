//! Doomking Balerdroch
use super::prelude::*;

pub const ID: u32 = 39185163;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card in Defense Position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Apply 1 of these effects (but you cannot apply that same effect of "Doomking Balerdroch" again this turn)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    effect::set_condition(d, e2, Some(condition(effcon)));
    effect::set_target(d, e2, Some(cost_target(efftg)));
    effect::set_operation(d, e2, Some(operation(effop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    let v = duel::get_chain_info(
        d,
        a.ev as u8,
        &[CHAININFO_TRIGGERING_TYPE, CHAININFO_TRIGGERING_RACE, CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2],
    );
    let trig_typ = v[0].int();
    let trig_race = v[1].int();
    let code1 = v[2].int();
    let code2 = v[3].int();
    (trig_typ & TYPE_MONSTER as i64) > 0 && (trig_race & RACE_ZOMBIE as i64) > 0 && code1 != ID as i64 && code2 != ID as i64
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let b1 = !duel::has_flag_effect(d, tp, ID, None) && duel::is_chain_disablable(d, a.ev as u8);
        let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
            && duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
        return b1 || b2;
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_EITHER as u8, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && duel::is_chain_disablable(d, a.ev as u8);
    let nv = aux::necro_valley_filter(|d, c| rmfilter(d, c));
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&nv), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    if !(b1 || b2) {
        return;
    }
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    if op == Some(1) {
        // Negate that effect
        duel::negate_effect(d, a.ev as u8);
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    } else if op == Some(2) {
        // Banish 1 monster from the field or GY
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
