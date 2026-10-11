//! G Golem Stubborn Menhir
use super::prelude::*;

pub const ID: u32 = 97053215;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_EARTH, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    card::enable_revive_limit(d, c);
    // Recover or Special Summon 1 EARTH from your GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, c, LOCATION_GRAVE)
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    card::is_monster(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_summonable_card(d, c)
        && (card::is_able_to_hand(d, c, None) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, e, tp, ft);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp, ft)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp, ft)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g.clone()), 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(g.clone()), 1, tp, LOCATION_GRAVE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, LOCATION_GRAVE as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    aux::to_hand_or_else(
        d,
        tc,
        tp,
        Some(&|d, _c| card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0),
        Some(&|d: &mut Duel, g: &CardSet| -> i32 {
            if let Some(tc) = group::get_first(d, g) {
                if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                    // Negate its effects
                    let c = effect::get_handler(d, e).nil();
                    let e1 = effect::create_effect(d, c);
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    effect::set_code(d, e1, EFFECT_DISABLE);
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                    card::register_effect(d, tc, e1, true);
                    let e2 = effect::create_effect(d, c);
                    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                    card::register_effect(d, tc, e2, true);
                    // Banish it if it leaves the field
                    let e3 = effect::create_effect(d, c);
                    effect::set_description(d, e3, 3300);
                    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                    effect::set_code(d, e3, EFFECT_LEAVE_FIELD_REDIRECT);
                    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                    effect::set_reset(d, e3, RESET_EVENT | RESETS_REDIRECT, 1);
                    effect::set_value(d, e3, LOCATION_REMOVED as i64);
                    card::register_effect(d, tc, e3, true);
                }
            }
            duel::special_summon_complete(d)
        }),
        Some(aux::stringid(ID, 1)),
    );
}
