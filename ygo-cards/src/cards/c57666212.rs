//! Kuraz the Light Monarch
use super::prelude::*;

pub const ID: u32 = 57666212;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetOperation(s.disatt)
    effect::set_operation(d, e3, Some(operation(disatt)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,2,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local sg=tg:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
    // sg=Duel.GetOperatedGroup()
    let sg = duel::get_operated_group(d);
    // local d1=0
    let mut d1: u32 = 0;
    // local d2=0
    let mut d2: u32 = 0;
    // local tc=sg:GetFirst()
    let _ = group::get_first(d, &sg);
    // for tc in aux.Next(sg) do
    for tc in group::iter(d, &sg) {
        // if tc then (the iterator only yields cards)
        // if tc:IsPreviousControler(0) then d1=d1+1
        if card::is_previous_controler(d, tc, 0) {
            d1 += 1;
        } else {
            // else d2=d2+1 end
            d2 += 1;
        }
    }
    // if d1>0 and Duel.SelectYesNo(0,aux.Stringid(id,1)) then Duel.Draw(0,d1,REASON_EFFECT) end
    if d1 > 0 && duel::select_yes_no(d, 0, aux::stringid(ID, 1)) {
        duel::draw(d, 0, d1, REASON_EFFECT);
    }
    // if d2>0 and Duel.SelectYesNo(1,aux.Stringid(id,1)) then Duel.Draw(1,d2,REASON_EFFECT) end
    if d2 > 0 && duel::select_yes_no(d, 1, aux::stringid(ID, 1)) {
        duel::draw(d, 1, d2, REASON_EFFECT);
    }
}
/// `s.disatt(e,tp,eg,ep,ev,re,r,rp)`
fn disatt(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
