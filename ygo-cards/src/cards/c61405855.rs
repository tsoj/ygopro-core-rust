//! Sword of Dragon's Soul
use super::prelude::*;

pub const ID: u32 = 61405855;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_WARRIOR))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_WARRIOR))), None, None, None, None, None, None);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(700)
    effect::set_value(d, e2, 700);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetCode(EVENT_BATTLED)
    effect::set_code(d, e4, EVENT_BATTLED);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let c = card::get_equip_target(d, h).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return bc and bc:IsRace(RACE_DRAGON)
    match bc {
        None => false,
        Some(bc) => card::is_race(d, bc, RACE_DRAGON),
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=e:GetHandler():GetEquipTarget():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let eqt = card::get_equip_target(d, h).nil();
    // (the trigger's condition guarantees a battle target, so a missing one is never reached)
    let bc = card::get_battle_target(d, eqt).nil();
    // Duel.SetTargetCard(bc)
    duel::set_target_card(d, bc);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(bc)), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local bc=Duel.GetFirstTarget()
    let bc = duel::first_target(d).nil();
    // if bc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, bc, a.e) {
        // Duel.Destroy(bc,REASON_EFFECT)
        duel::destroy(d, bc, REASON_EFFECT, None, None);
    }
}
