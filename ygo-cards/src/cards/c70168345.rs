//! Dark Magic Twin Burst
use super::prelude::*;

pub const ID: u32 = 70168345;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN_GIRL, CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_faceup(d, c) || !card::is_on_field(d, c)) && card::is_code(d, c, &[CARD_DARK_MAGICIAN_GIRL])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_code(d, chkc, &[CARD_DARK_MAGICIAN]);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_DARK_MAGICIAN]))), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&atkfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_DARK_MAGICIAN]))), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        let mut atk: i64 = 0;
        let g = duel::get_matching_group(d, Some(&atkfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
        // local bc=g:GetFirst()
        let _ = group::get_first(d, &g);
        for bc in group::iter(d, &g) {
            atk += card::get_attack(d, bc) as i64;
        }
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, atk);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
