//! Circle of the Fire Kings
use super::prelude::*;

pub const ID: u32 = 59388357;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn desfilter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_FIRE) && (ft > 0 || card::get_sequence(d, c) < 5)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        return ft > -1
            && duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, ft)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| desfilter(d, c, ft)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g2 = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1.clone()), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g2.clone()), 1, 0, 0);
    let first = group::get_first(d, &g1).map_or(Value::Nil, Value::Card);
    effect::set_label_object(d, a.e, first);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let targets = duel::get_first_target(d);
    let label = effect::get_label_object(d, a.e).as_card();
    let mut tc1 = targets.first().copied();
    let mut tc2 = targets.get(1).copied();
    if tc1 != label {
        std::mem::swap(&mut tc1, &mut tc2);
    }
    let tc1 = tc1.nil();
    if card::is_controler(d, tc1, a.tp)
        && card::is_relate_to_effect(d, tc1, a.e)
        && duel::destroy(d, tc1, REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, tc2.nil(), a.e)
    {
        duel::special_summon(d, tc2.nil(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
