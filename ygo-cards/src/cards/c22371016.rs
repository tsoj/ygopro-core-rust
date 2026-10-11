//! A.O.J Unknown Crusher
use super::prelude::*;

pub const ID: u32 = 22371016;

pub fn initial_effect(d: &mut Duel, c: CardId) {
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
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local a=Duel.GetAttacker()
        let mut at = duel::get_attacker(d);
        // if a==c then a=Duel.GetAttackTarget() end
        if at == Some(c) {
            at = duel::get_attack_target(d);
        }
        // e:SetLabelObject(a)
        match at {
            Some(x) => effect::set_label_object(d, e, Value::Card(x)),
            None => effect::set_label_object(d, e, Value::Nil),
        }
        // return a and a:IsAttribute(ATTRIBUTE_LIGHT)
        return match at {
            Some(x) => card::is_attribute(d, x, ATTRIBUTE_LIGHT),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,e:GetLabelObject(),1,0,0)
    let t = effect::get_label_object(d, e).as_card().map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, t, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // if tc:IsRelateToBattle() then Duel.Remove(tc,POS_FACEUP,REASON_EFFECT) end
    if card::is_relate_to_battle(d, tc) {
        duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
