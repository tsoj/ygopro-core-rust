//! Gizmek Uka, the Festive Fox of Fecundity
use super::prelude::*;

pub const ID: u32 = 90361289;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon self
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // special summon other
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    effect::set_operation(d, e2, Some(operation(spop2)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    // damage
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_condition(d, e4, Some(condition(damcon)));
    effect::set_target(d, e4, Some(cost_target(damtg)));
    effect::set_operation(d, e4, Some(operation(damop)));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e5, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_location(d, c, LOCATION_DECK), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && {
        let att = card::get_attribute(d, c);
        duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, att)), tp, LOCATION_HAND + LOCATION_DECK, 0, 1, NO_EX)
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, att: u32) -> bool {
    let atk = card::get_attack(d, c) as u32;
    card::is_defense(d, c, &[atk])
        && card::is_attack_above(d, c, 0)
        && card::is_defense_above(d, c, 0)
        && card::is_attribute(d, c, att)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && tgfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, e, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn spop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let att = card::get_attribute(d, tc);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, att)), tp, LOCATION_HAND + LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - tp);
    duel::set_target_param(d, 300);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 300);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let amt = info[1].int();
    duel::damage(d, p, amt, REASON_EFFECT, false, None);
}
