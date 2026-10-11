//! Dragonic Diagram
use super::prelude::*;

pub const ID: u32 = 13035077;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRUE_DRACO_KING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // All "True Draco" and "True King" monsters on the field gain 300 ATK/DEF
    let e1a = effect::create_effect(d, c);
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1a, LOCATION_FZONE);
    effect::set_target_range(d, e1a, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1a, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TRUE_DRACO_KING as u16]))));
    effect::set_value(d, e1a, 300);
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e1b, false);
    // The first time each Tribute Summoned "True Draco" or "True King" monster would be destroyed by battle each turn, it is not destroyed
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(e2_target)));
    effect::set_value_fn(d, e2, lua_fn(e2_value));
    card::register_effect(d, c, e2, false);
    // Destroy 1 other card you control or in your hand, and if you do, add 1 "True Draco" or "True King" card from your Deck to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn e2_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_tribute_summoned(d, c) && card::is_set_card(d, c, &[SET_TRUE_DRACO_KING as u16])
}
/// `function(e,re,r,rp) return (r&REASON_BATTLE)>0 and 1 or 0 end`
fn e2_value(d: &mut Duel, a: Args) -> i64 {
    let _ = d;
    let r = a.int(2);
    if (r & REASON_BATTLE as i64) > 0 { 1 } else { 0 }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TRUE_DRACO_KING as u16]) && card::is_able_to_hand(d, c, None)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, c.into());
    if a.chk == Some(0) {
        return !g.is_empty() && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    let tp = a.tp;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, tp, None, tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, exc).nil();
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let sg = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !sg.is_empty() {
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, sg);
        }
    }
}
