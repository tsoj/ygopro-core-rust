//! Binary Sorceress
use super::prelude::*;

pub const ID: u32 = 79016563;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| matfilter(d, x, sc, st, tp))), 2, Some(2), None, None, None);
    card::enable_revive_limit(d, c);
    // recover
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(reccon)));
    effect::set_target(d, e1, Some(cost_target(rectg)));
    effect::set_operation(d, e1, Some(operation(recop)));
    card::register_effect(d, c, e1, false);
    // negate
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    !card::is_type_ctx(d, c, TYPE_TOKEN, lc, sumtype, Some(tp))
}
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_mutual_linked_group(d, h);
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    a.ep != a.tp && lg.contains(tc) && card::get_battle_target(d, tc).is_some()
}
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, a.ev as u32);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, a.ev as i32);
    true
}
fn recop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    aux::stat_change_damage_step_condition(d) && card::get_mutual_linked_group_count(d, h) >= 2
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 2, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let first1 = group::get_first(d, &g1).nil();
    effect::set_label_object(d, a.e, Value::Card(first1));
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3));
    let _g2 = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, first1.into());
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let hc = effect::get_label_object(d, a.e).as_card().nil();
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let list = group::iter(d, &g);
    let mut tc = list.first().copied();
    if tc == Some(hc) {
        tc = list.get(1).copied();
    }
    if card::is_faceup(d, hc)
        && card::is_relate_to_effect(d, hc, a.e)
        && card::is_faceup(d, tc.nil())
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        let atk = card::get_attack(d, hc);
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        card::register_effect(d, hc, e1, false);
        if !card::is_immune_to_effect(d, hc, e1) {
            let e2 = effect::create_effect(d, h);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            effect::set_value(d, e2, lua_int(atk as f64 / 2.0));
            card::register_effect(d, tc.nil(), e2, false);
        }
    }
}
