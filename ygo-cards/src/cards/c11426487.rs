//! White Steed of the Floral Knights
use super::prelude::*;

pub const ID: u32 = 11426487;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_target_range(d, e1, POS_FACEUP_DEFENSE as u32, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_value(d, e1, POS_FACEUP_DEFENSE as i64);
    card::register_effect(d, c, e1, false);
    // Negate Attack + Pop
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let c = match a.card(1) {
        Some(c) => c,
        None => return true,
    };
    let tp = card::get_controler(d, c);
    let lv = aux::faceup_filter(|d, x| card::is_level_below(d, x, 2));
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&lv), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && card::is_can_be_special_summoned(d, c, a.e(0), 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.negtg(e,tp,...,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, tp);
    }
    if a.chk == Some(0) {
        let at = duel::get_attacker(d).nil();
        return card::is_controler(d, at, 1 - tp) && duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let tc = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc, 1, tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.negop(e,tp,...)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d);
    if duel::negate_attack(d) && card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_controler(d, tc.nil(), tp) {
        duel::destroy(d, tc.nil(), REASON_EFFECT, None, None);
    }
}
