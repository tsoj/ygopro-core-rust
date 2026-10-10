//! Genex Ally Birdman
use super::prelude::*;

pub const ID: u32 = 64034255;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon this card from hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,ft)`
fn cfilter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    card::is_faceup(d, c) && card::is_able_to_hand_as_cost(d, c) && (ft > 0 || card::get_sequence(d, c) < 5)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        return ft > -1 && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, ft)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, ft)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    if card::is_attribute(d, tc, ATTRIBUTE_WIND) {
        effect::set_label(d, e, &[1]);
    } else {
        effect::set_label(d, e, &[0]);
    }
    duel::send_to_hand(d, &g, None, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    if duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        if effect::get_label(d, e) == 1 {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, 500);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            card::register_effect(d, c, e1, false);
        }
        // Banish it if it leaves the field
        let e2 = effect::create_effect(d, c);
        effect::set_description(d, e2, 3300);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e2, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e2, true);
    }
    duel::special_summon_complete(d);
}
