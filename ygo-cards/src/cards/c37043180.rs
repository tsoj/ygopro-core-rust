//! D.D. Warrior
use super::prelude::*;

pub const ID: u32 = 37043180;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttackTarget()~=nil end
    if a.chk == Some(0) {
        return duel::get_attack_target(d).is_some();
    }
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dd = duel::get_attack_target(d);
    // local g=Group.FromCards(a,d)
    let g = group::create_group(&att.into_iter().chain(dd).collect::<Vec<CardId>>());
    // local rg=g:Filter(Card.IsRelateToBattle,nil)
    let rg = group::filter(d, &g, &|d, c| card::is_relate_to_battle(d, c), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rg,#rg,0,0)
    let n = rg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dd = duel::get_attack_target(d);
    // local g=Group.FromCards(a,d)
    let g = group::create_group(&att.into_iter().chain(dd).collect::<Vec<CardId>>());
    // local rg=g:Filter(Card.IsRelateToBattle,nil)
    let rg = group::filter(d, &g, &|d, c| card::is_relate_to_battle(d, c), NO_EX);
    // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
