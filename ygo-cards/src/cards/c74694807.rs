//! Re-Fusion
use super::prelude::*;

pub const ID: u32 = 74694807;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Remove
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 800);
    }
    duel::pay_lp_cost(d, a.tp, 800);
    true
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_FUSION) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, a.e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `function(e,c) return e:GetOwner()==c end`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    effect::get_owner(d, a.e(0)) == a.card(1)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        if duel::special_summon(d, CardSet::from_card(tc.nil()), 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        duel::equip(d, tp, c, tc.nil(), true, false);
        // Add Equip limit
        let e1 = effect::create_effect(d, tc.nil());
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_value_fn(d, e1, lua_fn(eqlimit));
        card::register_effect(d, c, e1, false);
    }
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, c);
    if card::is_reason(d, c, REASON_DESTROY) {
        if let Some(tc) = tc {
            if card::is_location(d, tc, LOCATION_MZONE) {
                duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
        }
    }
}
