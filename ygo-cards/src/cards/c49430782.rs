//! True Draco Heritage
use super::prelude::*;

pub const ID: u32 = 49430782;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRUE_DRACO_KING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Draw cards equal to the number of "True Draco" and "True King" card types sent to the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(cond_flag)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
    // Tribute Summon 1 "True Draco" or "True King" monster face-up
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(sumtg)));
    effect::set_operation(d, e3, Some(operation(sumop)));
    card::register_effect(d, c, e3, false);
    // Destroy 1 Spell/Trap on the field
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    effect::set_condition(d, e4, Some(condition(cond_prev_sz)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
    // Count the number of "True Draco" and "True King" card types sent to the GY each turn
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg();
    for tc in group::iter(d, &eg) {
        if card::is_previous_location(d, tc, LOCATION_ONFIELD) && card::is_set_card(d, tc, &[SET_TRUE_DRACO_KING as u16]) {
            let typ = (card::get_original_type(d, tc) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP)) as i64;
            let label = duel::get_flag_effect_label(d, 0, ID).first().copied();
            if label.is_none_or(|l| typ & l == 0) {
                let new_label = match label {
                    Some(l) => typ | l,
                    None => typ,
                };
                duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
                duel::set_flag_effect_label(d, 0, ID, new_label as i32);
            }
        }
    }
}
fn cond_flag(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_flag_effect(d, 0, ID) > 0
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let ct = duel::get_flag_effect(d, 0, ID);
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, a.tp, ct as u32);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct as i32);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let ct = duel::get_flag_effect(d, 0, ID) as u32;
    duel::draw(d, a.tp, ct, REASON_EFFECT);
}
fn sumfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TRUE_DRACO_KING as u16]) && card::is_summonable(d, c, true, None, 1, None)
}
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sumfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn sumop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&sumfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        duel::summon(d, a.tp, tc, true, None, 1, None);
    }
}
fn cond_prev_sz(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_SZONE)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
