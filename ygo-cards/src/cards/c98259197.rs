//! Ojamuscle
use super::prelude::*;

pub const ID: u32 = 98259197;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_OJAMA], listed_names: &[90140980], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
        && card::is_code(d, c, &[90140980])
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, cc| card::is_set_card(d, cc, &[SET_OJAMA as u16]))), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    let dg = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, cc| card::is_set_card(d, cc, &[SET_OJAMA as u16]))), 0, LOCATION_MZONE, LOCATION_MZONE, tc.into());
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    let dg = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, cc| card::is_set_card(d, cc, &[SET_OJAMA as u16]))), 0, LOCATION_MZONE, LOCATION_MZONE, tc.into());
    let ct = duel::destroy(d, dg, REASON_EFFECT, None, None);
    if ct > 0 {
        let tc = tc.nil();
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, ct as i64 * 1000);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
