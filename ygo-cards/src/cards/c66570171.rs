//! Necroworld Banshee
use super::prelude::*;

pub const ID: u32 = 66570171;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[4064256], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_FZONE, LOCATION_FZONE);
    // aux.TargetBoolFunction(Card.IsCode,4064256)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[4064256]))));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    card::register_effect(d, c, e2, false);
    // activate Zombie World
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e3, LOCATION_MZONE | LOCATION_GRAVE);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(actg)));
    effect::set_operation(d, e3, Some(operation(acop)));
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(4064256) and c:GetActivateEffect() and c:GetActivateEffect():IsActivatable(tp,true,true)
    if !card::is_code(d, c, &[4064256]) {
        return false;
    }
    match card::get_activate_effect(d, c).first().copied() {
        Some(ae) => effect::is_activatable(d, ae, tp, true, true),
        None => false,
    }
}
/// `s.actg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn actg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.acop(e,tp,eg,ep,ev,re,r,rp)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // Duel.ActivateFieldSpell(tc,e,tp,eg,ep,ev,re,r,rp)
    duel::activate_field_spell(d, tc, a, None);
}
