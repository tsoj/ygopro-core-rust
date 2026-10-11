//! Amphibious Bugroth MK-11
use super::prelude::*;

pub const ID: u32 = 4754691;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot attack directly if "Umi" isn't active
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(not_umicon)));
    card::register_effect(d, c, e1, false);
    // Gains 700 ATK if "Umi" isn't active
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 700);
    card::register_effect(d, c, e2, false);
    // Destroy 1 non-WATER monster on the field
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(umicon)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn umicon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_UMI]))), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        || duel::is_environment(d, CARD_UMI, None, None)
}
fn not_umicon(d: &mut Duel, a: &EffArgs) -> bool {
    !umicon(d, a)
}
fn non_water_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute_except(d, c, ATTRIBUTE_WATER, None, 0, None)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_attribute_except(d, chkc, ATTRIBUTE_WATER, None, 0, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&non_water_faceup), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&non_water_faceup), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
