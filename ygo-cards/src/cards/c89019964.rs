//! Unchained Soul of Disaster
use super::prelude::*;

pub const ID: u32 = 89019964;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_UNCHAINED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gain 300 ATK per each "Unchained" card in the GY
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
    // Link Summon 1 DARK Link monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Unchained" monster from the GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&|d, c| card::is_set_card(d, c, &[SET_UNCHAINED as u16])), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 300
}
fn lkfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_link_summonable(d, c, None, Some(mg.clone()), 2, 2)
}
fn tgfilter(d: &mut Duel, tc: CardId, c: CardId, tp: u8) -> bool {
    let mut mg = CardSet::from_card(c);
    group::add_card(&mut mg, &CardSet::from_card(tc));
    card::is_faceup(d, tc)
        && card::is_can_be_link_material(d, tc, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| lkfilter(d, x, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && tgfilter(d, chkc, h, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, h, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&|d, x| tgfilter(d, x, h, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_controler(d, c, tp) && card::is_faceup(d, c) {
        if let Some(tc) = tc {
            if card::is_controler(d, tc, 1 - tp) && card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
                let mut mg = CardSet::from_card(c);
                group::add_card(&mut mg, &CardSet::from_card(tc));
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                let g = duel::select_matching_card(d, tp, Some(&|d, x| lkfilter(d, x, &mg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
                let sc = group::get_first(d, &g);
                if let Some(sc) = sc {
                    duel::link_summon(d, tp, sc, None, Some(mg), 2, 2);
                }
            }
        }
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0 && card::is_previous_location(d, h, LOCATION_ONFIELD)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_UNCHAINED as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
