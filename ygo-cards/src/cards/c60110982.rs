//! Armityle the Chaos Phantasm - Phantom of Fury
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 60110982;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[6007213, 32491822, 69890967, 43378048], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(6007213), fusion::Mat::Code(32491822), fusion::Mat::Code(69890967)]);
    // Name change on the field
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value(d, e1, 43378048);
    card::register_effect(d, c, e1, false);
    // Give control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_CONTROL);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(cttg)));
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
    // Register the control switch
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_CANNOT_DISABLE + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e3, EVENT_CONTROL_CHANGED);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_operation(d, e3, Some(operation(regop)));
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,chkc) if chk==0 then return e:GetHandler():IsAbleToChangeControler() end ... end`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_change_controler(d, h);
    }
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(h.into()), 1, 0, 0);
    true
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        duel::get_control(d, c, 1 - a.tp, 0, 0, None, None);
    }
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end ... end`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, LOCATION_ONFIELD as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[43378048])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let rg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, 0, NO_EX);
    if duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_owner(d, h);
    let (e, id) = (a.e, ID);
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, p)), p, LOCATION_EXTRA, 0, NO_EX);
    if group::get_count(&g) > 0 && duel::select_yes_no(d, p, aux::stringid(id, 1)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
        duel::break_effect(d);
        duel::special_summon(d, sg, 0, p, p, true, false, POS_FACEUP as u8, None);
    }
}
