//! Called by the Grave
use super::prelude::*;

pub const ID: u32 = 24224830;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
            let c = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
            effect::set_target(d, e1, Some(effect_card_fn(distg)));
            let codes: Vec<i64> = card::get_original_code_rule(d, tc).into_iter().map(|x| x as i64).collect();
            effect::set_label(d, e1, &codes);
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
            duel::register_effect(d, e1, tp);

            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
            effect::set_condition(d, e2, Some(condition(discon)));
            effect::set_operation(d, e2, Some(operation(disop)));
            let codes: Vec<i64> = card::get_original_code_rule(d, tc).into_iter().map(|x| x as i64).collect();
            effect::set_label(d, e2, &codes);
            effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
            duel::register_effect(d, e2, tp);
        }
    }
}
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let code = effect::get_label(d, e);
    let codes = card::get_original_code_rule(d, c);
    let code1 = codes.first().map(|&x| x as i64);
    let code2 = codes.get(1).map(|&x| x as i64);
    code1 == Some(code) || code2 == Some(code)
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let code = effect::get_label(d, a.e);
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    let codes = card::get_original_code_rule(d, h);
    let code1 = codes.first().map(|&x| x as i64);
    let code2 = codes.get(1).map(|&x| x as i64);
    effect::is_monster_effect(d, re) && (code1 == Some(code) || code2 == Some(code))
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    duel::negate_effect(d, a.ev as u8);
}
