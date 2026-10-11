//! Cupid Volley
use super::prelude::*;

pub const ID: u32 = 11851647;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // lv
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // recover
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.reccon)
    effect::set_condition(d, e2, Some(condition(reccon)));
    // e2:SetTarget(s.rectg)
    effect::set_target(d, e2, Some(cost_target(rectg)));
    // e2:SetOperation(s.recop)
    effect::set_operation(d, e2, Some(operation(recop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeckAsCost(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck_as_cost(d, a.tp, 1);
    }
    // local ct={}
    let mut ct: Vec<u64> = Vec::new();
    // for i=3,1,-1 do
    for i in (1..=3u32).rev() {
        // if Duel.IsPlayerCanDiscardDeckAsCost(tp,i) then table.insert(ct,i) end
        if duel::is_player_can_discard_deck_as_cost(d, a.tp, i) {
            ct.push(i as u64);
        }
    }
    // if #ct==1 then
    if ct.len() == 1 {
        // Duel.DiscardDeck(tp,ct[1],REASON_COST)
        duel::discard_deck(d, a.tp, ct[0] as u16, REASON_COST);
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
        // local ac=Duel.AnnounceNumber(tp,table.unpack(ct))
        let ac = duel::announce_number(d, a.tp, &ct).map(|x| x.0).nil();
        // Duel.DiscardDeck(tp,ac,REASON_COST)
        duel::discard_deck(d, a.tp, ac as u16, REASON_COST);
        // e:SetLabel(ac)
        effect::set_label(d, a.e, &[ac as i64]);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local ct=e:GetLabel()
        let ct = effect::get_label(d, a.e);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // e1:SetValue(ct)
        effect::set_value(d, e1, ct);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.reccon(e,tp,eg,ep,ev,re,r,rp)`
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_DESTROY)~=0 and e:GetHandler():IsPreviousControler(tp)
    //     and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    (a.r & REASON_DESTROY) != 0 && card::is_previous_controler(d, h, a.tp) && card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1500)
    duel::set_target_param(d, 1500);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,1500)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 1500);
    true
}
/// `s.recop(e,tp,eg,ep,ev,re,r,rp)`
fn recop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
