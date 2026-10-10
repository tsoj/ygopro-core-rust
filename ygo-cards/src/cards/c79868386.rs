//! Fist of Fate
use super::prelude::*;

pub const ID: u32 = 79868386;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_OBELISK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE | CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CANNOT_NEGATE | EFFECT_FLAG_CANNOT_INACTIVATE, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_original_code_rule(d, x, &[CARD_OBELISK]))), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn negatable(d: &mut Duel, x: CardId) -> bool {
    card::is_negatable_monster(d, x)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&negatable), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&negatable), tp, 0, LOCATION_MZONE, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g.clone()), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let g = duel::select_matching_card(d, tp, Some(&negatable), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        let c = effect::get_handler(d, e).nil();
        // Negate its effects
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        card::register_effect(d, tc, e2, false);
        duel::adjust_instantly(d, Some(c));
        if !card::is_immune_to_effect(d, tc, e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
            // Negate effects of cards with the same original name
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_FIELD);
            effect::set_code(d, e3, EFFECT_DISABLE);
            effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
            effect::set_target(d, e3, Some(effect_card_fn(distg)));
            let codes: Vec<i64> = card::get_original_code_rule(d, tc).into_iter().map(|x| x as i64).collect();
            effect::set_label(d, e3, &codes);
            effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e3, tp);
            let e4 = effect::create_effect(d, c);
            effect::set_type(d, e4, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
            effect::set_condition(d, e4, Some(condition(discon)));
            effect::set_operation(d, e4, Some(operation(disop)));
            effect::set_label(d, e4, &codes);
            effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e4, tp);
            // Destroy all Spells/Traps the opponent controls
            let sg = duel::get_matching_group(d, Some(&|d, x| card::is_spell_trap(d, x)), tp, 0, LOCATION_ONFIELD, NO_EX);
            if duel::is_main_phase(d, None) && duel::is_turn_player(d, tp) && !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
                duel::break_effect(d);
                duel::destroy(d, sg, REASON_EFFECT, None, None);
            }
        }
    }
}
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let code = effect::get_label(d, e);
    let v = card::get_original_code_rule(d, c);
    v.first().map(|&x| x as i64) == Some(code) || v.get(1).map(|&x| x as i64) == Some(code)
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let code = effect::get_label(d, a.e);
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    let v = card::get_original_code_rule(d, h);
    let code1 = v.first().map(|&x| x as i64);
    let code2 = v.get(1).map(|&x| x as i64);
    effect::is_monster_effect(d, re) && (code1 == Some(code) || code2 == Some(code))
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    duel::negate_effect(d, a.ev as u8);
}
