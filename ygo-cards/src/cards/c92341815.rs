//! Starlit Papillon
use super::prelude::*;

pub const ID: u32 = 92341815;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ADVENTURER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    aux::add_equip_procedure(d, c, Some(0), None, None, None, None, None, None, None);
    // Decrease ATK by 500 per monster that mentions "Adventurer Token"
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_value(atkval));
    card::register_effect(d, c, e1, false);
    // Equip itself to "Adventurer Token"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(eqcon)));
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    effect::set_operation(d, e2, Some(operation(eqop)));
    card::register_effect(d, c, e2, false);
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::lists_code(d, c, &[TOKEN_ADVENTURER])
}
/// `function(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&atkfilter), p, LOCATION_MZONE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64 * -500
}
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::check_unique_on_field(d, c, a.tp, None, None)
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[TOKEN_ADVENTURER])
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && eqfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return card::is_relate_to_effect(d, c, a.e)
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&eqfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    let _ = duel::select_target(d, a.tp, Some(&eqfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e)
            && card::is_faceup(d, tc)
            && card::is_code(d, tc, &[TOKEN_ADVENTURER])
            && card::is_controler(d, tc, a.tp)
            && card::check_unique_on_field(d, c, a.tp, None, None)
        {
            duel::equip(d, a.tp, c, tc, true, false);
        }
    }
}
