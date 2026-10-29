//! Future Glow
use super::prelude::*;

pub const ID: u32 = 64107820;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_PSYCHIC))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_PSYCHIC))));
    // e2:SetValue(s.val)
    effect::set_value_fn(d, e2, effect_value(val));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e1:SetLabelObject(e2)
    effect::set_label_object(d, e1, Value::Effect(e2));
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PSYCHIC) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_PSYCHIC) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(g:GetFirst():GetLevel()*200)
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first) as i64 * 200;
    effect::set_label(d, a.e, &[lv]);
    // e:GetLabelObject():SetLabel(0)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[0]);
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(e:GetLabel())
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let label = effect::get_label(d, a.e);
    effect::set_label(d, lo, &[label]);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetLabel()
    effect::get_label(d, e)
}
