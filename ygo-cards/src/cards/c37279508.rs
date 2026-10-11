//! No.37 Hope Woven Dragon Spider Shark
use super::prelude::*;

pub const ID: u32 = 37279508;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(37), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // atk down
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        effect::set_value(d, e1, -1000);
        card::register_effect(d, tc, e1, false);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_DESTROY) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp) && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
