//! Book of Life
use super::prelude::*;

pub const ID: u32 = 2204140;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_ZOMBIE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g1 = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g1, 1, 0, 0);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g2 = duel::select_target(d, tp, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g2, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tg1 = duel::get_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON).and_then(|x| x.op_cards);
    let tg2 = duel::get_operation_info(d, 0, CATEGORY_REMOVE).and_then(|x| x.op_cards);
    let tc1 = group::get_first(d, tg1.as_ref().nil()).nil();
    if card::is_relate_to_effect(d, tc1, e) && card::is_race(d, tc1, RACE_ZOMBIE) {
        duel::special_summon(d, tg1.clone().nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    let tc2 = group::get_first(d, tg2.as_ref().nil()).nil();
    if card::is_relate_to_effect(d, tc2, e) {
        duel::remove(d, tg2.clone().nil(), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
