//! Giant Rex
use super::prelude::*;

pub const ID: u32 = 80280944;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot direct attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    card::register_effect(d, c, e1, false);
    // spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (c, tp) = (effect::get_handler(d, a.e).nil(), a.tp);
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_DINOSAUR))), tp, LOCATION_REMOVED, 0, NO_EX);
        if !g.is_empty() {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
            effect::set_range(d, e1, LOCATION_MZONE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, g.len() as i64 * 200);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            card::register_effect(d, c, e1, false);
        }
    }
    duel::special_summon_complete(d);
}
