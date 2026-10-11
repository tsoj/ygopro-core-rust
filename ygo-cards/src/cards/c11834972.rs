//! Laval Blaster
//! ラヴァル・ガンナー
use super::prelude::*;

pub const ID: u32 = 11834972;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LAVAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LAVAL) and c:GetCode()~=id
    card::is_set_card(d, c, &[SET_LAVAL as u16]) && card::get_code(d, c)[0] != ID
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeckAsCost(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck_as_cost(d, a.tp, 1);
    }
    // local ct={}
    let mut ct: Vec<u64> = Vec::new();
    // for i=5,1,-1 do
    for i in (1..=5u32).rev() {
        // if Duel.IsPlayerCanDiscardDeckAsCost(tp,i) then table.insert(ct,i) end
        if duel::is_player_can_discard_deck_as_cost(d, a.tp, i) {
            ct.push(i as u64);
        }
    }
    // if #ct==1 then
    if ct.len() == 1 {
        // Duel.DiscardDeck(tp,ct[1],REASON_COST)
        duel::discard_deck(d, a.tp, ct[0] as u16, REASON_COST);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
        // local ac=Duel.AnnounceNumber(tp,table.unpack(ct))
        let ac = duel::announce_number(d, a.tp, &ct).nil().0;
        // Duel.DiscardDeck(tp,ac,REASON_COST)
        duel::discard_deck(d, a.tp, ac as u16, REASON_COST);
    }
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // e:SetLabel(g:FilterCount(Card.IsSetCard,nil,SET_LAVAL)*200)
    let n = group::filter_count(d, &g, &|d, c| card::is_set_card(d, c, &[SET_LAVAL as u16]), NO_EX) as i64;
    effect::set_label(d, a.e, &[n * 200]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e1:SetValue(e:GetLabel())
        let label = effect::get_label(d, a.e);
        effect::set_value(d, e1, label);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
