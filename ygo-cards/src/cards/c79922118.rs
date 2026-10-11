//! Gingerbread House
use super::prelude::*;

pub const ID: u32 = 79922118;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::get_attack(d, c) + 600 >= 2500
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        // local chk=false
        let mut chk = false;
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // if tc:UpdateAttack(600,nil,c)==600 then chk=true end
            if card::update_attack(d, tc, 600, None, Some(c), None) == 600 {
                chk = true;
            }
        }
        // local dg=g:Filter(Card.IsAttackAbove,nil,2500)
        let dg = group::filter(d, &g, &|d, x| card::is_attack_above(d, x, 2500), NO_EX);
        // if chk and #dg>0 then
        if chk && !dg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local ct=Duel.Destroy(dg,REASON_EFFECT)
            let ct = duel::destroy(d, dg, REASON_EFFECT, None, None);
            // if ct>0 then Duel.Recover(tp,ct*500,REASON_EFFECT) end
            if ct > 0 {
                duel::recover(d, a.tp, ct as i64 * 500, REASON_EFFECT, false, None);
            }
        }
    }
}
