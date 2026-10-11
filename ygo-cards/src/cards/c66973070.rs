//! Dreadscythe Harvester
use super::prelude::*;

pub const ID: u32 = 66973070;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() (the ex argument of the release functions)
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsRace,1,false,nil,e:GetHandler(),RACE_INSECT) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| card::is_race(d, c, RACE_INSECT as u64)), 1, 1, false, None, c.into());
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,Card.IsRace,1,1,false,nil,e:GetHandler(),RACE_INSECT)
    let sg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| card::is_race(d, c, RACE_INSECT as u64)), 1, 1, false, None, c.into());
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
