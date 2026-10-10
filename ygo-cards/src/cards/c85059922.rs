//! Master of Chaos
use super::prelude::*;

pub const ID: u32 = 85059922;
pub const INFO: ScriptInfo = ScriptInfo {
    material: &[CARD_DARK_MAGICIAN],
    listed_names: &[CARD_DARK_MAGICIAN],
    listed_series: &[SET_CHAOS],
    material_setcode: &[SET_CHAOS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Fusion Materials
    // Fusion.AddProcMix(c,true,true,CARD_DARK_MAGICIAN,s.ffilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![fusion::Mat::Code(CARD_DARK_MAGICIAN), fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp)))],
    );
    // Special Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| card::is_fusion_summoned(d, effect::get_handler(d, a.e).nil()))));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Banish all opponent monsters
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(rmcost)));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
    // Add Spell to hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(condition(thcon)));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    card::is_type_ctx(d, c, TYPE_RITUAL, Some(fc), sumtype, Some(tp))
        && (card::is_set_card_ctx(d, c, &[SET_CHAOS as u16], Some(fc), sumtype, Some(tp)) || card::is_set_card_ctx(d, c, &[SET_NUMBER_C as u16], Some(fc), sumtype, Some(tp)))
}
/// `s.attfilter(c)`
fn attfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) || card::is_attribute(d, c, ATTRIBUTE_DARK)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    attfilter(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.attcheck(sg,tp)`
fn attcheck(d: &mut Duel, sg: &CardSet, tp: u8) -> bool {
    group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == 2
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, sg.into())
}
fn release_check(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet) -> (bool, bool) {
    (attcheck(d, sg, tp), false)
}
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, tp, Some(&|d, c| attfilter(d, c)), 2, 2, false, Some(&release_check), NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    let g = aux::select_release_group_cost(d, tp, Some(&|d, c| attfilter(d, c)), 2, 2, false, Some(&release_check), NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 1 - tp, LOCATION_MZONE as i32);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_fusion_summoned(d, c) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
