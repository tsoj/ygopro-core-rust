//! Garmr of the Nordic Beasts
use super::prelude::*;

pub const ID: u32 = 88283496;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetCondition(s.retcon)
    effect::set_condition(d, e1, Some(condition(retcon)));
    // e1:SetTarget(s.rettg)
    effect::set_target(d, e1, Some(cost_target(rettg)));
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc==e:GetHandler() then tc=Duel.GetAttackTarget() end
    let h = effect::get_handler(d, a.e).nil();
    if tc == Some(h) {
        tc = duel::get_attack_target(d);
    }
    // if not tc then return false end
    let Some(tc) = tc else {
        return false;
    };
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc);
    // return lv>0 and lv<=4 and not tc:IsStatus(STATUS_BATTLE_DESTROYED)
    lv > 0 && lv <= 4 && !card::is_status(d, tc, STATUS_BATTLE_DESTROYED)
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk ==0 then return e:GetLabelObject():IsAbleToHand() end
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, tc, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetLabelObject(),0,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(tc)), 0, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():IsRelateToBattle() then
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_battle(d, tc) {
        // Duel.SendtoHand(e:GetLabelObject(),nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
