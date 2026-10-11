//! Blackwing - Jin the Rain Shadow
use super::prelude::*;

pub const ID: u32 = 38562933;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local d=Duel.GetAttackTarget()
    let mut tgt = duel::get_attack_target(d);
    // if d==c then d=Duel.GetAttacker() end
    if tgt == Some(c) {
        tgt = duel::get_attacker(d);
    }
    // e:SetLabelObject(d)
    match tgt {
        Some(t) => effect::set_label_object(d, a.e, Value::Card(t)),
        None => effect::set_label_object(d, a.e, Value::Nil),
    }
    // return d and d:IsFaceup() and d:IsDefenseBelow(c:GetAttack())
    match tgt {
        Some(t) => {
            if !card::is_faceup(d, t) {
                return false;
            }
            let atk = card::get_attack(d, c);
            card::is_defense_below(d, t, atk)
        }
        None => false,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetLabelObject(),1,0,0)
    let lo = effect::get_label_object(d, a.e).as_card().map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, lo, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local d=e:GetLabelObject()
    let tgt = effect::get_label_object(d, a.e).as_card();
    // if c:IsFaceup() and c:IsRelateToEffect(e) and d:IsRelateToBattle() and d:IsDefenseBelow(c:GetAttack()) then
    if !card::is_faceup(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tgt = tgt.nil();
    if !card::is_relate_to_battle(d, tgt) {
        return;
    }
    let atk = card::get_attack(d, c);
    if card::is_defense_below(d, tgt, atk) {
        // Duel.Destroy(d,REASON_EFFECT)
        duel::destroy(d, tgt, REASON_EFFECT, None, None);
    }
}
