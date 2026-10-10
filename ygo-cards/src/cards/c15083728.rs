//! House of Adhesive Tape
use super::prelude::*;

pub const ID: u32 = 15083728;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tc;
    }
    if a.chk == Some(0) {
        if a.ep == a.tp {
            return false;
        }
        let tc = tc.nil();
        return card::is_faceup(d, tc) && card::get_defense(d, tc) <= 500 && card::is_on_field(d, tc) && card::is_can_be_effect_target(d, tc, Some(a.e));
    }
    duel::set_target_card(d, &eg);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::get_defense(d, tc) <= 500 {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
