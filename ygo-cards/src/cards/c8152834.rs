//! Tarai
use super::prelude::*;

pub const ID: u32 = 8152834;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy a monster and Special Summon itself from the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Register a flag on a monster that activates an effect on the field
    aux::global_check(d, ID, move |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_CHAIN_SOLVED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    if effect::is_monster_effect(d, re) && card::is_relate_to_effect(d, rc, re) && loc == LOCATION_MZONE as i64 {
        card::register_flag_effect(d, rc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let controller = card::get_controler(d, c);
    let seq = card::get_sequence(d, c);
    card::is_faceup(d, c)
        && card::get_flag_effect(d, c, ID) != 0
        && duel::get_m_zone_count(d, controller, c.into(), Some(tp), Some(LOCATION_REASON_TOFIELD), Some(1 << seq)).0 > 0
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MMZONE) && desfilter(d, chkc, tp);
    }
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| desfilter(d, x, tp)), tp, LOCATION_MMZONE, LOCATION_MMZONE, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, Some(&|d, x| desfilter(d, x, tp)), tp, LOCATION_MMZONE, LOCATION_MMZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 && card::is_relate_to_effect(d, c, e) {
        let zone = 1u32 << card::get_previous_sequence(d, tc);
        let p = if card::is_previous_controler(d, tc, tp) { tp } else { 1 - tp };
        duel::special_summon(d, c, 0, tp, p, false, false, POS_FACEUP as u8, Some(zone));
    }
}
