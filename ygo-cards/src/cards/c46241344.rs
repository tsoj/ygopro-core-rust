//! Brotherhood of the Fire Fist - Eagle
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 46241344;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FORMATION, SET_FIRE_FIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    card::set_spsummon_once(d, c, ID);
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_BEASTWARRIOR, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // negate cost
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_COST_REPLACE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_value_fn(d, e1, lua_fn(repval));
    card::register_effect(d, c, e1, false);
    // to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    card::register_effect(d, c, e2, false);
}
// function s.repval(base,extracon,e,...)
fn repval(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(2)).nil();
    if !(card::is_set_card(d, c, &[SET_FIRE_FIST as u16]) && card::is_monster(d, c)) {
        return false;
    }
    // extracon(base,e,...)
    let mut full = vec![a.v(0).clone(), a.v(2).clone()];
    full.extend(a.0.iter().skip(3).cloned());
    match a.v(1) {
        Value::Fn(f) => f.call(d, &full).truthy(),
        _ => false,
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
        && card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16])
        && card::is_able_to_hand(d, c, None)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
}
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_FIRE_FIST as u16]) && card::is_able_to_grave(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::break_effect(d);
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
