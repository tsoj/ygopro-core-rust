//! Card Trooper
use super::prelude::*;

pub const ID: u32 = 85087012;

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
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
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
    // for i=3,1,-1 do
    //     if Duel.IsPlayerCanDiscardDeckAsCost(tp,i) then
    //         table.insert(ct,i)
    //     end
    // end
    let mut ct: Vec<u64> = Vec::new();
    for i in (1..=3u32).rev() {
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
        let ac = duel::announce_number(d, a.tp, &ct).nil().0;
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
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // e1:SetValue(ct*500)
        effect::set_value(d, e1, ct * 500);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_DESTROY)~=0 and e:GetHandler():IsPreviousControler(tp)
    //     and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let c = effect::get_handler(d, a.e).nil();
    (a.r & REASON_DESTROY) != 0 && card::is_previous_controler(d, c, a.tp) && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
