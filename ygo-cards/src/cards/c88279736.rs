//! Robbin' Goblin
use super::prelude::*;

pub const ID: u32 = 88279736;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // handes
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and eg:GetFirst():IsControler(tp)
    if a.ep == a.tp {
        return false;
    }
    let g = a.eg.clone().nil();
    let tc = group::get_first(d, &g).nil();
    card::is_controler(d, tc, a.tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,0,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetFieldGroup(ep,LOCATION_HAND,0,nil)
    let g = duel::get_field_group(d, a.ep, LOCATION_HAND, 0);
    // local sg=g:RandomSelect(ep,1)
    let sg = group::random_select(d, &g, a.ep, 1);
    // Duel.SendtoGrave(sg,REASON_DISCARD|REASON_EFFECT)
    duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
}
