//! Rainbow Veil
use super::prelude::*;

pub const ID: u32 = 12735388;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // --disable
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.discon1)
    effect::set_condition(d, e3, Some(condition(discon1)));
    // e3:SetOperation(s.disop1)
    effect::set_operation(d, e3, Some(operation(disop1)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.discon1(e,tp,eg,ep,ev,re,r,rp)`
fn discon1(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let Some(ec) = card::get_equip_target(d, h) else { return false };
    // return ec and (ec==Duel.GetAttacker() or ec==Duel.GetAttackTarget()) and ec:GetBattleTarget()
    (Some(ec) == duel::get_attacker(d) || Some(ec) == duel::get_attack_target(d)) && card::get_battle_target(d, ec).is_some()
}
/// `s.disop1(e,tp,eg,ep,ev,re,r,rp)`
fn disop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetEquipTarget():GetBattleTarget()
    let eqt = card::get_equip_target(d, c).nil();
    let tc = card::get_battle_target(d, eqt).nil();
    // c:CreateRelation(tc,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    card::create_relation(d, c, tc, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetProperty(EFFECT_FLAG_OWNER_RELATE)
    effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // e1:SetCondition(s.discon2)
    effect::set_condition(d, e1, Some(condition(discon2)));
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // e2:SetOperation(s.disop2)
    effect::set_operation(d, e2, Some(operation(disop2)));
    // e2:SetLabelObject(tc)
    effect::set_label_object(d, e2, Value::Card(tc));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.discon2(e)`
fn discon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetOwner():IsRelateToCard(e:GetHandler())
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_relate_to_card(d, owner, h)
}
/// `s.disop2(e,tp,eg,ep,ev,re,r,rp)`
fn disop2(d: &mut Duel, a: &EffArgs) {
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // if loc==LOCATION_MZONE and re:GetHandler()==e:GetLabelObject() then
    if loc == LOCATION_MZONE as i64 {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re);
        if rh == effect::get_label_object(d, a.e).as_card() {
            // Duel.NegateEffect(ev)
            duel::negate_effect(d, a.ev as u8);
        }
    }
}
