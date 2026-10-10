//! Call of the Haunted
use super::prelude::*;

pub const ID: u32 = 97077563;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate this card by targeting 1 monster in your GY; Special Summon that target in Attack Position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // When this card leaves the field, destroy that monster
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    effect::set_operation(d, e2, Some(operation(e2_op)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e3, Some(operation(mondesop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
    // When that monster is destroyed, destroy this card
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_condition(d, e4, Some(condition(selfdescon)));
    effect::set_operation(d, e4, Some(operation(e4_op)));
    card::register_effect(d, c, e4, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None) && card::is_relate_to_effect(d, c, a.e) {
        card::set_card_target(d, c, tc);
    }
    duel::special_summon_complete(d);
}
fn e2_op(d: &mut Duel, a: &EffArgs) {
    // e:SetLabel(e:GetHandler():IsDisabled() and 1 or 0)
    let h = effect::get_handler(d, a.e).nil();
    let v = if card::is_disabled(d, h) { 1 } else { 0 };
    effect::set_label(d, a.e, &[v]);
}
fn mondesop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():GetLabel()~=0 then return end
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, lo) != 0 {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_first_card_target(d, h) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
fn selfdescon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    match card::get_first_card_target(d, h) {
        Some(tc) => {
            let eg = a.eg.clone().nil();
            group::is_contains(&eg, tc) && card::is_reason(d, tc, REASON_DESTROY)
        }
        None => false,
    }
}
fn e4_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
