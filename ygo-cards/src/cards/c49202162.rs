//! Black Luster Soldier - Soldier of Chaos
use super::prelude::*;

pub const ID: u32 = 49202162;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    card::enable_revive_limit(d, c);
    link::add_procedure(d, c, None, 3, Some(3), Some(group_check(lcheck)), None, None);
    // Gain Effect
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e1, lua_fn(matcheck));
    card::register_effect(d, c, e1, false);
    // Effect when destroying by battle
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e2, Some(aux::bdocon()));
    effect::set_target(d, e2, Some(cost_target(dtarget)));
    effect::set_operation(d, e2, Some(operation(doperation)));
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`: `g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    group::check_different_property(d, g, &code_list)
}
fn code_list(d: &mut Duel, c: CardId) -> Vec<i64> {
    card::get_code(d, c).iter().map(|x| *x as i64).collect()
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsLevelAbove(7)
    card::is_monster(d, c) && card::is_level_above(d, c, 7)
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, a: Args) {
    let c = a.c(1);
    let g = card::get_material(d, c);
    if group::is_exists(d, &g, &filter, 1, NO_EX) {
        // Your opponent cannot target with card effects.
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(ID, 0));
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_value_fn(d, e1, aux::tgoval());
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        card::register_effect(d, c, e1, false);
        // Your opponent cannot destroy with card effects.
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        effect::set_value_fn(d, e2, lua_fn(indval));
        card::register_effect(d, c, e2, false);
    }
}
/// `s.indval(e,re,tp)`
fn indval(d: &mut Duel, a: Args) -> bool {
    a.p(2) != effect::get_handler_player(d, a.e(0))
}
/// `s.dtarget(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dtarget(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let b3 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    let op = duel::select_effect(d, a.tp, &[(true, aux::stringid(ID, 1)), (true, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).unwrap_or(0) as i64;
    effect::set_label(d, a.e, &[op]);
    effect::set_category(d, a.e, 0);
    if op == 3 {
        effect::set_category(d, a.e, CATEGORY_REMOVE);
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, a.tp, 0);
    }
    true
}
/// `s.doperation(e,tp,eg,ep,ev,re,r,rp)`
fn doperation(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let op = effect::get_label(d, a.e);
    if op == 1 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 1500);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
    } else if op == 2 {
        let e1 = effect::create_effect(d, c);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        effect::set_value(d, e1, 1);
        effect::set_label(d, e1, &[duel::get_turn_count(d, None) as i64]);
        effect::set_condition(d, e1, Some(lua_fn(atcon)));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 2);
        card::register_effect(d, c, e1, false);
    } else if op == 3 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
/// `s.atcon(e,tp)`
fn atcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let _c = effect::get_handler(d, e).nil();
    duel::get_turn_count(d, None) as i64 > effect::get_label(d, e)
}
