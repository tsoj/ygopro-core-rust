//! Unibird
use super::prelude::*;

pub const ID: u32 = 21296383;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
fn cfilter(d: &mut Duel, c: CardId, lv: i64, ft: i64) -> bool {
    card::is_faceup(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::get_original_level(d, c) as i64 >= lv
        && (ft > 0 || card::get_sequence(d, c) < 5)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, a.tp)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&sg, chkc) && card::is_level_below(d, chkc, effect::get_label(d, e) as u32);
    }
    if sg.is_empty() {
        return false;
    }
    let (_mg, mlv) = group::get_min_group(d, &sg, &|d, c| card::get_level(d, c) as i64).nil();
    let h = effect::get_handler(d, e).nil();
    let elv = card::get_original_level(d, h) as i64;
    let lv = if elv >= mlv { 1 } else { mlv - elv };
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    if a.chk == Some(0) {
        return ft > -1
            && card::is_able_to_remove_as_cost(d, h, None)
            && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, lv, ft)), a.tp, LOCATION_MZONE, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let mut g = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter(d, c, lv, ft)), a.tp, LOCATION_MZONE, 0, 1, 1, false, h.into()).nil();
    let tc = group::get_first(d, &g).nil();
    let slv = elv + card::get_level(d, tc) as i64;
    group::add_card(&mut g, &CardSet::from_card(h));
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    effect::set_label(d, e, &[slv]);
    let g = group::filter_select(d, &sg, a.tp, &|d, c| card::is_level_below(d, c, slv as u32), 1, 1, false, NO_EX).nil();
    duel::set_target_card(d, &g);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g.clone()), 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
