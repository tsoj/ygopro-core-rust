//! Amazoness Heirloom
use super::prelude::*;

pub const ID: u32 = 79965360;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZONESS as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_AMAZONESS))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_AMAZONESS as u16]))), None, None, None, None, None, None);
    // destroy rep
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_COUNT);
    // e3:SetValue(s.valcon)
    effect::set_value_fn(d, e3, lua_fn(valcon));
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
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
/// `s.valcon(e,re,r,rp)`
fn valcon(_d: &mut Duel, a: Args) -> bool {
    // return r==REASON_BATTLE
    a.int(2) as u32 == REASON_BATTLE
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget()==Duel.GetAttacker() and Duel.GetAttackTarget()
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h) == duel::get_attacker(d) && duel::get_attack_target(d).is_some()
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttackTarget(),1,0,0)
    let at = duel::get_attack_target(d).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, at, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d).nil();
    // if tc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, tc) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
