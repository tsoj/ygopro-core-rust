//! Spell Strider
use super::prelude::*;

pub const ID: u32 = 93966624;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_spell(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&rmfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, effect::get_handler(d, e).nil(), e, 0, tp, false, false, None, None, None);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g1 = duel::select_target(d, tp, Some(&rmfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g2 = duel::select_target(d, tp, Some(&rmfilter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    let mut g1 = g1.nil();
    group::add_card(&mut g1, &g2.nil());
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g1), 2, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(effect::get_handler(d, e).nil())), 1, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let g = duel::get_target_cards(d, e).nil();
    if !g.is_empty() && duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
        let c = effect::get_handler(d, e).nil();
        if !card::is_relate_to_effect(d, c, e) {
            return;
        }
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
